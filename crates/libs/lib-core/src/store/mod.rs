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
    
    // SQLite concurrency optimizations for high-load scenarios
    // WAL mode: enables concurrent reads during writes
    let _ = conn.execute("PRAGMA journal_mode=WAL;", ()).await;
    // busy_timeout: wait up to 30 seconds when database is locked (instead of failing immediately)
    // This is critical for 100+ concurrent agents writing simultaneously
    let _ = conn.execute("PRAGMA busy_timeout=30000;", ()).await;
    // synchronous=NORMAL: good balance of safety and performance with WAL
    let _ = conn.execute("PRAGMA synchronous=NORMAL;", ()).await;
    // cache_size: increase cache to reduce disk I/O (negative = KB, so -64000 = 64MB)
    let _ = conn.execute("PRAGMA cache_size=-64000;", ()).await;

    // Apply all migrations in order
    // Note: SQLite's IF NOT EXISTS makes this idempotent for table creation
    let migrations = [
        include_str!("../../../../../migrations/001_initial_schema.sql"),
        include_str!("../../../../../migrations/002_agent_capabilities.sql"),
        include_str!("../../../../../migrations/003_tool_metrics.sql"),
        include_str!("../../../../../migrations/004_attachments.sql"),
    ];

    for migration in &migrations {
        conn.execute_batch(migration).await?;
    }

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