use axum::{
    extract::{State},
    routing::{post, get},
    Json, Router,
    response::{IntoResponse, Response}, // Added Response import
};
use serde::{Deserialize, Serialize};
use crate::AppState; // Use AppState from main.rs
use crate::tools;
use crate::error::ServerError; // Added ServerError import

pub fn routes() -> Router<AppState> { // State is generic type for Router in axum 0.6
    Router::new()
        .route("/api/health", get(tools::health_check))
        .route("/api/project/ensure", post(tools::ensure_project))
        .route("/api/agent/register", post(tools::register_agent))
        .route("/api/message/send", post(tools::send_message))
        .route("/api/inbox", post(tools::list_inbox))
        .route("/api/projects", get(tools::list_all_projects))
        .route("/api/projects/:project_slug/agents", get(tools::list_all_agents_for_project))
        .route("/api/messages/:message_id", get(tools::get_message))
        .route("/api/file_reservations/paths", post(tools::file_reservation_paths))
}

// Re-export common types
pub use tools::EnsureProjectPayload;
pub use tools::RegisterAgentPayload;
pub use tools::SendMessagePayload;
pub use tools::ListInboxPayload;
pub use tools::ProjectResponse; // Added
pub use tools::ListAgentsPayload; // Added
pub use tools::AgentResponse; // Added
pub use tools::GetMessagePayload; // Added
pub use tools::MessageResponse; // Added
pub use tools::FileReservationPathsPayload;
pub use tools::FileReservationPathsResponse;
