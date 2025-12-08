use anyhow::Result;
use axum::{
    routing::{get, post},
    Router, Server, // Added Server import
};
use lib_core::ModelManager;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;
use crate::error::ServerError; // Added ServerError import



mod api;
mod tools;
mod error; // Added error module

// --- Application State
#[derive(Clone)]
struct AppState {
    mm: ModelManager,
}

#[tokio::main]
async fn main() -> std::result::Result<(), ServerError> {
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
        .merge(api::routes()) // Merge API routes
        .route("/", get(root_handler))
        .route("/mcp", post(mcp_handler)) // MCP endpoint (will be integrated into API later)
        .with_state(app_state);

    let port = 8000; // Hardcode port for server binary
    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let std_listener = listener.into_std().map_err(|e| ServerError::from(e))?; // Convert to std::net::TcpListener
    std_listener.set_nonblocking(true).map_err(|e| ServerError::from(e))?; // Set non-blocking
    axum::Server::from_tcp(std_listener)?.serve(app.into_make_service()).await?;

    Ok(())
}

async fn root_handler() -> &'static str {
    "MCP Agent Mail Server is running!"
}

async fn mcp_handler() -> &'static str {
    // TODO: Integrate mcp-protocol-sdk here
    "MCP endpoint - not yet implemented"
}


