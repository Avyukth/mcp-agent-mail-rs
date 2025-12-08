use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::Result;
use crate::store::git_store;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub project_id: i64,
    pub sender_id: i64,
    pub thread_id: Option<String>,
    pub subject: String,
    pub body_md: String,
    pub importance: String,
    pub ack_required: bool,
    pub created_ts: NaiveDateTime,
    // attachments is JSON
}

#[derive(Deserialize, Serialize)]
pub struct MessageForCreate {
    pub project_id: i64,
    pub sender_id: i64,
    pub recipient_ids: Vec<i64>,
    pub subject: String,
    pub body_md: String,
    pub thread_id: Option<String>,
    pub importance: Option<String>,
}

pub struct MessageBmc;

impl MessageBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, msg_c: MessageForCreate) -> Result<i64> {
        let db = mm.db();
        let repo_root = &mm.repo_root;

        // 1. Insert into DB
        let thread_id = msg_c.thread_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let importance = msg_c.importance.unwrap_or("normal".to_string());

        // Helper to serialize attachments (empty for now)
        let attachments_json = "[]"; 

        let mut stmt = db.prepare(
            r#"
            INSERT INTO messages (project_id, sender_id, thread_id, subject, body_md, importance, attachments)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING id
            "#
        ).await?;
        
        let mut rows = stmt.query((
            msg_c.project_id,
            msg_c.sender_id,
            thread_id.as_str(),
            msg_c.subject.as_str(),
            msg_c.body_md.as_str(),
            importance.as_str(),
            attachments_json
        )).await?;

        let id = if let Some(row) = rows.next().await? {
            row.get::<i64>(0)?
        } else {
            return Err(crate::Error::InvalidInput("Failed to create message".into()));
        };

        // 2. Insert Recipients
        for recipient_id in &msg_c.recipient_ids {
             db.execute(
                "INSERT INTO message_recipients (message_id, agent_id) VALUES (?, ?)",
                (id, *recipient_id)
            )
            .await?;
        }

        // 3. Git Operations
        // Need Project Slug
        let mut stmt = db.prepare("SELECT slug FROM projects WHERE id = ?").await?;
        let mut rows = stmt.query([msg_c.project_id]).await?;
        let project_slug: String = if let Some(row) = rows.next().await? {
            row.get(0)?
        } else {
            return Err(crate::Error::ProjectNotFound(msg_c.project_id));
        };

        // Need Sender Name
        let mut stmt = db.prepare("SELECT name FROM agents WHERE id = ?").await?;
        let mut rows = stmt.query([msg_c.sender_id]).await?;
        let sender_name: String = if let Some(row) = rows.next().await? {
            row.get(0)?
        } else {
            return Err(crate::Error::AgentNotFound(msg_c.sender_id));
        };

        // Need Recipient Names
        let mut recipient_names = Vec::new();
        for recipient_id in &msg_c.recipient_ids {
             let mut stmt = db.prepare("SELECT name FROM agents WHERE id = ?").await?;
             let mut rows = stmt.query([*recipient_id]).await?;
             if let Some(row) = rows.next().await? {
                 recipient_names.push(row.get::<String>(0)?);
             }
        }

        // Construct paths
        let now = chrono::Utc::now();
        let y_dir = now.format("%Y").to_string();
        let m_dir = now.format("%m").to_string();
        let created_iso = now.format("%Y-%m-%dT%H-%M-%SZ").to_string();
        
        let subject_slug = slug::slugify(&msg_c.subject);
        let filename = format!("{}__{}__{}.md", created_iso, subject_slug, id);

        let project_root = PathBuf::from("projects").join(&project_slug);
        let canonical_path = project_root.join("messages").join(&y_dir).join(&m_dir).join(&filename);
        
        let outbox_path = project_root.join("agents").join(&sender_name).join("outbox").join(&y_dir).join(&m_dir).join(&filename);

        let mut inbox_paths = Vec::new();
        for recipient_name in &recipient_names {
            inbox_paths.push(
                project_root.join("agents").join(recipient_name).join("inbox").join(&y_dir).join(&m_dir).join(&filename)
            );
        }

        // Content
        let frontmatter = serde_json::json!({
            "id": id,
            "project": project_slug,
            "from": sender_name,
            "to": recipient_names,
            "subject": msg_c.subject,
            "thread_id": thread_id,
            "created": created_iso,
            "importance": importance,
        });
        let content = format!("---json\n{}\n---\n\n{}", serde_json::to_string_pretty(&frontmatter)?, msg_c.body_md);

        // Commit
        let repo = git_store::open_repo(repo_root)?;
        let commit_msg = format!("mail: {} -> {} | {}", sender_name, recipient_names.join(", "), msg_c.subject);

        let workdir = repo.workdir().ok_or(crate::Error::InvalidInput("No workdir".into()))?;
        
        fn write_file(root: &std::path::Path, rel: &std::path::Path, content: &str) -> Result<()> {
             let full = root.join(rel);
             if let Some(p) = full.parent() {
                 std::fs::create_dir_all(p)?;
             }
             std::fs::write(full, content)?;
             Ok(())
        }

        write_file(workdir, &canonical_path, &content)?;
        write_file(workdir, &outbox_path, &content)?;
        for inbox_path in &inbox_paths {
            write_file(workdir, inbox_path, &content)?;
        }

        // Collect all paths to commit
        let mut all_paths = vec![canonical_path, outbox_path];
        all_paths.extend(inbox_paths);

        git_store::commit_paths(
            &repo,
            &all_paths,
            &commit_msg,
            "mcp-bot",
            "mcp-bot@localhost",
        )?;

        Ok(id)
    }
}
