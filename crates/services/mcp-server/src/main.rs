use axum::routing::{get, post};
use axum::Router;
use lib_core::ModelManager;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

use crate::error::ServerError;

mod api;
mod error;
mod tools;

// --- Application State
#[derive(Clone)]
pub struct AppState {
    pub mm: ModelManager,
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
        .merge(api::routes())
        .route("/", get(root_handler))
        .route("/mcp", post(mcp_handler))
        .with_state(app_state);

    let port = 8000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("listening on {}", addr);

    // Axum 0.8+ uses axum::serve() directly with tokio listener
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


