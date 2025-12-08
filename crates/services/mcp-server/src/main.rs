use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use lib_core::ModelManager;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Initialize ModelManager
    let mm = ModelManager::new().await?;
    let app_state = AppState { mm };

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/mcp", post(mcp_handler)) // MCP endpoint
        .with_state(app_state);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root_handler() -> &'static str {
    "MCP Agent Mail Server is running!"
}

async fn mcp_handler() -> &'static str {
    // TODO: Integrate mcp-protocol-sdk here
    "MCP endpoint - not yet implemented"
}

#[derive(Clone)]
struct AppState {
    mm: ModelManager,
}

// Basic health check
async fn health_check() -> &'static str {
    "OK"
}
