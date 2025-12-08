use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::Result;
use crate::store::git_store;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use std::path::PathBuf;
use uuid::Uuid;
use serde_json::Value;

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
    pub attachments: Vec<Value>, // Use Vec<Value> for attachments
    pub sender_name: String, // Added sender_name for inbox display
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

        let stmt = db.prepare(
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
        let stmt = db.prepare("SELECT slug FROM projects WHERE id = ?").await?;
        let mut rows = stmt.query([msg_c.project_id]).await?;
        let project_slug: String = if let Some(row) = rows.next().await? {
            row.get(0)?
        } else {
            return Err(crate::Error::ProjectNotFound(format!("ID: {}", msg_c.project_id)));
        };

        // Need Sender Name
        let stmt = db.prepare("SELECT name FROM agents WHERE id = ?").await?;
        let mut rows = stmt.query([msg_c.sender_id]).await?;
        let sender_name: String = if let Some(row) = rows.next().await? {
            row.get(0)?
        } else {
            return Err(crate::Error::AgentNotFound(format!("ID: {}", msg_c.sender_id)));
        };

        // Need Recipient Names
        let mut recipient_names = Vec::new();
        for recipient_id in &msg_c.recipient_ids {
            let stmt = db.prepare("SELECT name FROM agents WHERE id = ?").await?;
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
        let mut all_paths = vec![canonical_path.clone()]; // Canonical path for commit
        all_paths.push(outbox_path.clone());
        all_paths.extend(inbox_paths.clone());

        // Convert PathBuf to AsRef<Path>
        let all_paths_as_ref: Vec<&std::path::Path> = all_paths.iter().map(|p| p.as_path()).collect();

        git_store::commit_paths(
            &repo,
            &all_paths_as_ref,
            &commit_msg,
            "mcp-bot",
            "mcp-bot@localhost",
        )?;

        Ok(id)
    }

    pub async fn list_inbox_for_agent(_ctx: &Ctx, mm: &ModelManager, project_id: i64, agent_id: i64, limit: i64) -> Result<Vec<Message>> {
        let db = mm.db();
        let stmt = db.prepare(
            r#"
            SELECT
                m.id, m.project_id, m.sender_id, ag.name as sender_name, m.thread_id, m.subject, m.body_md,
                m.importance, m.ack_required, m.created_ts, m.attachments
            FROM messages AS m
            JOIN message_recipients AS mr ON m.id = mr.message_id
            JOIN agents AS ag ON m.sender_id = ag.id
            WHERE mr.agent_id = ? AND m.project_id = ?
            ORDER BY m.created_ts DESC
            LIMIT ?
            "#
        ).await?;

        let mut rows = stmt.query((agent_id, project_id, limit)).await?;
        let mut messages = Vec::new();

        while let Some(row) = rows.next().await? {
            let id: i64 = row.get(0)?;
            let project_id: i64 = row.get(1)?;
            let sender_id: i64 = row.get(2)?;
            let sender_name: String = row.get(3)?;
            let thread_id: Option<String> = row.get(4)?;
            let subject: String = row.get(5)?;
            let body_md: String = row.get(6)?;
            let importance: String = row.get(7)?;
            let ack_required: bool = row.get(8)?;
            let created_ts_str: String = row.get(9)?;
            let created_ts = NaiveDateTime::parse_from_str(&created_ts_str, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default();

            let attachments_str: String = row.get(10)?;
            let attachments: Vec<Value> = serde_json::from_str(&attachments_str)?;

            messages.push(Message {
                id,
                project_id,
                sender_id,
                sender_name,
                thread_id,
                subject,
                body_md,
                importance,
                ack_required,
                created_ts,
                attachments,
            });
        }
        Ok(messages)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, message_id: i64) -> Result<Message> {
        let db = mm.db();
        let stmt = db.prepare(
            r#"
            SELECT
                m.id, m.project_id, m.sender_id, ag.name as sender_name, m.thread_id, m.subject, m.body_md,
                m.importance, m.ack_required, m.created_ts, m.attachments
            FROM messages AS m
            JOIN agents AS ag ON m.sender_id = ag.id
            WHERE m.id = ?
            "#
        ).await?;

        let mut rows = stmt.query([message_id]).await?;

        if let Some(row) = rows.next().await? {
            let id: i64 = row.get(0)?;
            let project_id: i64 = row.get(1)?;
            let sender_id: i64 = row.get(2)?;
            let sender_name: String = row.get(3)?;
            let thread_id: Option<String> = row.get(4)?;
            let subject: String = row.get(5)?;
            let body_md: String = row.get(6)?;
            let importance: String = row.get(7)?;
            let ack_required: bool = row.get(8)?;
            let created_ts_str: String = row.get(9)?;
            let created_ts = NaiveDateTime::parse_from_str(&created_ts_str, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default();

            let attachments_str: String = row.get(10)?;
            let attachments: Vec<Value> = serde_json::from_str(&attachments_str)?;

            Ok(Message {
                id,
                project_id,
                sender_id,
                sender_name,
                thread_id,
                subject,
                body_md,
                importance,
                ack_required,
                created_ts,
                attachments,
            })
        } else {
            Err(crate::Error::MessageNotFound(message_id))
        }
    }
}
