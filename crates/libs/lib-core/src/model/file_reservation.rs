use crate::model::ModelManager;
use crate::Result;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReservation {
    pub id: i64,
    pub project_id: i64,
    pub agent_id: i64,
    pub path_pattern: String,
    pub exclusive: bool,
    pub reason: String,
    pub created_ts: NaiveDateTime,
    pub expires_ts: NaiveDateTime,
    pub released_ts: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReservationForCreate {
    pub project_id: i64,
    pub agent_id: i64,
    pub path_pattern: String,
    pub exclusive: bool,
    pub reason: String,
    pub expires_ts: NaiveDateTime,
}

pub struct FileReservationBmc;

impl FileReservationBmc {
    pub async fn create(_ctx: &crate::Ctx, mm: &ModelManager, fr_c: FileReservationForCreate) -> Result<i64> {
        let db = mm.db();
        
        let stmt = db.prepare(
            r#"
            INSERT INTO file_reservations (project_id, agent_id, path_pattern, exclusive, reason, expires_ts)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id
            "#
        ).await?;
        
        // Format datetime as string for SQLite
        let expires_ts_str = fr_c.expires_ts.format("%Y-%m-%d %H:%M:%S").to_string();

        let mut rows = stmt.query((
            fr_c.project_id,
            fr_c.agent_id,
            fr_c.path_pattern,
            fr_c.exclusive,
            fr_c.reason,
            expires_ts_str,
        )).await?;

        let id = if let Some(row) = rows.next().await? {
            row.get::<i64>(0)?
        } else {
            return Err(crate::Error::InvalidInput("Failed to create file reservation".into()));
        };

        // TODO: Write to Git (will be added in a later step when implementing the tool)

        Ok(id)
    }

    pub async fn list_active_for_project(_ctx: &crate::Ctx, mm: &ModelManager, project_id: i64) -> Result<Vec<FileReservation>> {
        let db = mm.db();
        // Select active (not released). Checking expiry is better done in app logic or filter
        let stmt = db.prepare(
            r#"
            SELECT id, project_id, agent_id, path_pattern, exclusive, reason, created_ts, expires_ts, released_ts
            FROM file_reservations 
            WHERE project_id = ? AND released_ts IS NULL
            ORDER BY created_ts DESC
            "#
        ).await?;
        let mut rows = stmt.query([project_id]).await?;
        
        let mut reservations = Vec::new();
        while let Some(row) = rows.next().await? {
            reservations.push(Self::from_row(row)?);
        }
        Ok(reservations)
    }
    
    pub async fn get(_ctx: &crate::Ctx, mm: &ModelManager, id: i64) -> Result<FileReservation> {
        let db = mm.db();
        let stmt = db.prepare(
            r#"
            SELECT id, project_id, agent_id, path_pattern, exclusive, reason, created_ts, expires_ts, released_ts
            FROM file_reservations 
            WHERE id = ?
            "#
        ).await?;
        let mut rows = stmt.query([id]).await?;
        
        if let Some(row) = rows.next().await? {
            Ok(Self::from_row(row)?)
        } else {
            Err(crate::Error::FileReservationNotFound(format!("{}", id)))
        }
    }

    pub async fn release(_ctx: &crate::Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();
        let now = chrono::Utc::now().naive_utc();
        let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let stmt = db.prepare(
            r#"
            UPDATE file_reservations SET released_ts = ? WHERE id = ?
            "#
        ).await?;
        
        stmt.execute((now_str, id)).await?;
        Ok(())
    }

    fn from_row(row: libsql::Row) -> Result<FileReservation> {
        let created_ts_str: String = row.get(6).unwrap_or_default();
        let expires_ts_str: String = row.get(7).unwrap_or_default();
        let released_ts_str: Option<String> = row.get(8).unwrap_or_default();

        let created_ts = NaiveDateTime::parse_from_str(&created_ts_str, "%Y-%m-%d %H:%M:%S")
            .unwrap_or_default();
        let expires_ts = NaiveDateTime::parse_from_str(&expires_ts_str, "%Y-%m-%d %H:%M:%S")
            .unwrap_or_default();
        
        let released_ts = if let Some(s) = released_ts_str {
            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok()
        } else {
            None
        };

        Ok(FileReservation {
            id: row.get(0)?,
            project_id: row.get(1)?,
            agent_id: row.get(2)?,
            path_pattern: row.get(3)?,
            exclusive: row.get(4)?,
            reason: row.get(5)?,
            created_ts,
            expires_ts,
            released_ts,
        })
    }
}
