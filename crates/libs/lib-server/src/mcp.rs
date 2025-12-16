//! MCP (Model Context Protocol) HTTP handler module
//!
//! This module provides the HTTP/SSE endpoint for MCP protocol at `/mcp`.
//! It integrates lib-mcp's AgentMailService with Axum's routing system.

use std::sync::Arc;
use axum::{
    body::Body,
    http::{Request, Response},
    routing::any_service,
    Router,
};
use rmcp::transport::streamable_http_server::{
    tower::{StreamableHttpService, StreamableHttpServerConfig},
    session::local::LocalSessionManager,
};
use tower::ServiceExt;
use lib_mcp::tools::AgentMailService;
use lib_core::ModelManager;

use crate::AppState;

/// Create the MCP service for the /mcp route
///
/// This creates a tower-compatible service that handles MCP JSON-RPC 2.0 requests
/// over HTTP/SSE. The service is stateful (using LocalSessionManager) and creates
/// a new AgentMailService for each connection, sharing the ModelManager with main server.
fn create_mcp_service(mm: ModelManager) -> StreamableHttpService<AgentMailService> {
    // Create session manager for stateful connections
    let session_manager = Arc::new(LocalSessionManager::default());

    // Configure the HTTP server
    let config = StreamableHttpServerConfig::default();

    // Wrap ModelManager in Arc for sharing across connections
    let mm = Arc::new(mm);

    // Create a service factory that creates a new AgentMailService for each connection.
    // Uses the shared ModelManager to avoid migration conflicts.
    let service_factory = move || -> Result<AgentMailService, std::io::Error> {
        Ok(AgentMailService::new_with_mm(mm.clone()))
    };

    // Create the StreamableHttpService (tower-compatible)
    StreamableHttpService::new(
        service_factory,
        session_manager,
        config,
    )
}

/// Get the MCP route for integration into the main router
///
/// This returns an Axum Router that handles both GET (SSE stream) and POST (tool calls)
/// on the /mcp endpoint. Uses the ModelManager from AppState to share database connection.
pub fn mcp_routes(mm: ModelManager) -> Router<AppState> {
    let mcp_service = create_mcp_service(mm);

    // Wrap the MCP service to convert body types
    let wrapped_service = tower::service_fn(move |req: Request<Body>| {
        let svc = mcp_service.clone();
        async move {
            // Call the MCP service
            let response = svc.oneshot(req).await?;
            // Convert BoxBody to axum::body::Body
            let (parts, body) = response.into_parts();
            let body = Body::new(body);
            Ok::<_, std::convert::Infallible>(Response::from_parts(parts, body))
        }
    });

    Router::new()
        .route("/mcp", any_service(wrapped_service))
}
