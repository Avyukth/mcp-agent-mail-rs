use libsql::{Builder, Connection};
use crate::Result;
use std::path::PathBuf;

pub type Db = Connection;
pub mod git_store; // Add this line back

pub async fn new_db_pool() -> Result<Db> {
    // Ensure data directory exists
    let db_path = PathBuf::from("data/mcp_agent_mail.db");
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let _db_url = format!("file:{}", db_path.display());
    let db = Builder::new_local(db_path).build().await?;
    let conn = db.connect()?;
    
    // Basic manual migration for now since sqlx::migrate is gone
    // In a real app, use a migration crate or custom logic
    let _ = conn.execute("PRAGMA journal_mode=WAL;", ()).await;
    
    // Read and apply initial schema manually
    let schema = include_str!("../../../../../migrations/001_initial_schema.sql");
    conn.execute_batch(schema).await?;

    Ok(conn)
}

/// Helper function to get a connection for executing queries.
pub async fn get_db_connection(_db_url: &str) -> Result<Connection> {
    // For local file, we just open it
    // This function signature might need adjustment for Turso remote later
    // For now, just reusing new_db_pool logic or similar
    let db = Builder::new_local(PathBuf::from("data/mcp_agent_mail.db")).build().await?;
    let conn = db.connect()?;
    Ok(conn)
}