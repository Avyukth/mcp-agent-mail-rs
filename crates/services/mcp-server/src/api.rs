use axum::routing::{get, post};
use axum::Router;

use crate::tools;
use crate::AppState;

pub fn routes() -> Router<AppState> {
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
