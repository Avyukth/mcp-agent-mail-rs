//! File attachment management for projects.
//!
//! This module handles file attachments that can be shared between agents
//! in a project. Attachment metadata is stored in the database while the
//! actual files are stored on disk.
//!
//! # Storage
//!
//! - **Database**: Stores metadata (filename, path, media type, size)
//! - **Disk**: Actual file content at `stored_path`
//!
//! # Example
//!
//! ```no_run
//! use lib_core::model::attachment::{AttachmentBmc, AttachmentForCreate};
//! use lib_core::model::ModelManager;
//! use lib_core::ctx::Ctx;
//!
//! # async fn example() -> lib_core::Result<()> {
//! let mm = ModelManager::new().await?;
//! let ctx = Ctx::root_ctx();
//!
//! // Create attachment record (file already written to disk)
//! let attachment = AttachmentForCreate {
//!     project_id: 1,
//!     filename: "report.pdf".to_string(),
//!     stored_path: "/data/uploads/abc123.pdf".to_string(),
//!     media_type: "application/pdf".to_string(),
//!     size_bytes: 1024,
//! };
//! let id = AttachmentBmc::create(&ctx, &mm, attachment).await?;
//! # Ok(())
//! # }
//! ```

use crate::model::ModelManager;
use crate::{Ctx, Result};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// File attachment metadata.
///
/// Represents a file that has been uploaded and stored. The actual file
/// content is stored at `stored_path` on disk.
///
/// # Fields
///
/// - `id` - Database primary key
/// - `project_id` - Associated project
/// - `filename` - Original filename
/// - `stored_path` - Path to file on disk
/// - `media_type` - MIME type (e.g., "application/pdf")
/// - `size_bytes` - File size in bytes
/// - `created_ts` - Upload timestamp
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Attachment {
    /// Database primary key.
    pub id: i64,
    /// Associated project ID.
    pub project_id: i64,
    /// Original filename.
    pub filename: String,
    /// Path to file on disk.
    pub stored_path: String,
    /// MIME type.
    pub media_type: String,
    /// File size in bytes.
    pub size_bytes: i64,
    /// Upload timestamp.
    pub created_ts: String,
}

/// Input data for creating an attachment record.
///
/// The file must already be written to disk at `stored_path` before
/// creating the database record.
#[derive(Deserialize)]
pub struct AttachmentForCreate {
    /// Project to associate with.
    pub project_id: i64,
    /// Original filename.
    pub filename: String,
    /// Path where file is stored.
    pub stored_path: String,
    /// MIME type of the file.
    pub media_type: String,
    /// File size in bytes.
    pub size_bytes: i64,
}

/// Backend Model Controller for Attachment operations.
///
/// Manages file attachments associated with projects. Files are stored
/// on disk and metadata is tracked in the database.
pub struct AttachmentBmc;

impl AttachmentBmc {
    /// Creates a new attachment record.
    ///
    /// **Note**: This only creates the database record. The actual file
    /// must be written to disk separately (typically by the API layer).
    ///
    /// # Arguments
    /// * `_ctx` - Request context
    /// * `mm` - ModelManager
    /// * `attachment_c` - Attachment metadata
    ///
    /// # Returns
    /// The created attachment's database ID
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        attachment_c: AttachmentForCreate,
    ) -> Result<i64> {
        let db = mm.db();
        let now = chrono::Utc::now().naive_utc();
        let created_ts = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let stmt = db.prepare(
            "INSERT INTO attachments (project_id, filename, stored_path, media_type, size_bytes, created_ts) VALUES (?, ?, ?, ?, ?, ?) RETURNING id"
        ).await?;

        let mut rows = stmt
            .query((
                attachment_c.project_id,
                attachment_c.filename,
                attachment_c.stored_path,
                attachment_c.media_type,
                attachment_c.size_bytes,
                created_ts,
            ))
            .await?;

        if let Some(row) = rows.next().await? {
            Ok(row.get(0)?)
        } else {
            Err(crate::Error::InvalidInput(
                "Failed to create attachment".into(),
            ))
        }
    }

    /// Retrieves an attachment by its database ID.
    ///
    /// # Arguments
    /// * `_ctx` - Request context
    /// * `mm` - ModelManager
    /// * `id` - Attachment ID
    ///
    /// # Returns
    /// The attachment metadata
    ///
    /// # Errors
    /// Returns `Error::NotFound` if attachment doesn't exist
    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Attachment> {
        let db = mm.db();
        let stmt = db.prepare("SELECT id, project_id, filename, stored_path, media_type, size_bytes, created_ts FROM attachments WHERE id = ?").await?;
        let mut rows = stmt.query([id]).await?;

        if let Some(row) = rows.next().await? {
            Ok(Self::from_row(row)?)
        } else {
            Err(crate::Error::NotFound)
        }
    }

    /// Lists all attachments for a project.
    ///
    /// # Arguments
    /// * `_ctx` - Request context
    /// * `mm` - ModelManager
    /// * `project_id` - Project database ID
    ///
    /// # Returns
    /// Vector of attachments (newest first)
    pub async fn list_by_project(
        _ctx: &Ctx,
        mm: &ModelManager,
        project_id: i64,
    ) -> Result<Vec<Attachment>> {
        let db = mm.db();
        let stmt = db.prepare("SELECT id, project_id, filename, stored_path, media_type, size_bytes, created_ts FROM attachments WHERE project_id = ? ORDER BY id DESC").await?;
        let mut rows = stmt.query([project_id]).await?;

        let mut res = Vec::new();
        while let Some(row) = rows.next().await? {
            res.push(Self::from_row(row)?);
        }
        Ok(res)
    }

    fn from_row(row: libsql::Row) -> Result<Attachment> {
        Ok(Attachment {
            id: row.get(0)?,
            project_id: row.get(1)?,
            filename: row.get(2)?,
            stored_path: row.get(3)?,
            media_type: row.get(4)?,
            size_bytes: row.get(5)?,
            created_ts: row.get(6)?,
        })
    }
}
