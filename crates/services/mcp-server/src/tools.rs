use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use lib_core::model::file_reservation::{FileReservationBmc, FileReservationForCreate};
use lib_core::{self, Ctx};
use serde::{Deserialize, Serialize};

use crate::AppState;

// --- health_check ---
#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
    timestamp: String,
}

pub async fn health_check(_state: State<AppState>) -> crate::error::Result<Response> {
    Ok(Json(HealthCheckResponse {
        status: "ok".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    }).into_response())
}

// --- ensure_project ---
#[derive(Deserialize)]
pub struct EnsureProjectPayload {
    pub human_key: String,
}

#[derive(Serialize)]
pub struct EnsureProjectResponse {
    pub project_id: i64,
    pub slug: String,
}

pub async fn ensure_project(State(app_state): State<AppState>, Json(payload): Json<EnsureProjectPayload>) -> crate::error::Result<Response> {
    let ctx = Ctx::root_ctx(); // For now, use a root context
    let mm = &app_state.mm;

    // Call lib-core ProjectBmc to ensure project exists
    let project = match lib_core::model::project::ProjectBmc::get_by_human_key(&ctx, mm, &payload.human_key).await {
        Ok(p) => p,
        Err(e) => {
            if let lib_core::Error::ProjectNotFound(_) = e {
                // If not found, create it. Generate a slug here based on human_key.
                let slug = lib_core::utils::slugify(&payload.human_key);
                let _id = lib_core::model::project::ProjectBmc::create(&ctx, mm, &slug, &payload.human_key).await?;
                lib_core::model::project::ProjectBmc::get_by_human_key(&ctx, mm, &payload.human_key).await?
            } else {
                return Err(e.into());
            }
        }
    };

    Ok(Json(EnsureProjectResponse {
        project_id: project.id,
        slug: project.slug,
    }).into_response())
}

// --- register_agent ---
#[derive(Deserialize)]
pub struct RegisterAgentPayload {
    pub project_slug: String,
    pub name: String,
    pub program: String,
    pub model: String,
    pub task_description: String,
}

#[derive(Serialize)]
pub struct RegisterAgentResponse {
    pub agent_id: i64,
    pub name: String,
}

pub async fn register_agent(State(app_state): State<AppState>, Json(payload): Json<RegisterAgentPayload>) -> crate::error::Result<Response> {
    let ctx = Ctx::root_ctx();
    let mm = &app_state.mm;

    let project = lib_core::model::project::ProjectBmc::get_by_slug(&ctx, mm, &payload.project_slug).await?;

    let agent_c = lib_core::model::agent::AgentForCreate {
        project_id: project.id,
        name: payload.name.clone(),
        program: payload.program,
        model: payload.model,
        task_description: payload.task_description,
    };

    let agent_id = lib_core::model::agent::AgentBmc::create(&ctx, mm, agent_c).await?;

    Ok(Json(RegisterAgentResponse {
        agent_id,
        name: payload.name,
    }).into_response())
}

// --- send_message ---
#[derive(Deserialize)]
pub struct SendMessagePayload {
    pub project_slug: String,
    // Support both naming conventions for compatibility
    #[serde(alias = "from_agent_name")]
    pub sender_name: String,
    #[serde(alias = "to_agent_names")]
    pub recipient_names: Vec<String>,
    pub subject: String,
    pub body_md: String,
    pub thread_id: Option<String>,
    pub importance: Option<String>,
    #[serde(default)]
    pub ack_required: bool,
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    pub message_id: i64,
}

pub async fn send_message(State(app_state): State<AppState>, Json(payload): Json<SendMessagePayload>) -> crate::error::Result<Response> {
    let ctx = Ctx::root_ctx();
    let mm = &app_state.mm;

    let project = lib_core::model::project::ProjectBmc::get_by_slug(&ctx, mm, &payload.project_slug).await?;
    let sender = lib_core::model::agent::AgentBmc::get_by_name(&ctx, mm, project.id, &payload.sender_name).await?;

    let mut recipient_ids = Vec::new();
    for name in payload.recipient_names {
        let agent = lib_core::model::agent::AgentBmc::get_by_name(&ctx, mm, project.id, &name).await?;
        recipient_ids.push(agent.id);
    }

    let msg_c = lib_core::model::message::MessageForCreate {
        project_id: project.id,
        sender_id: sender.id,
        recipient_ids,
        subject: payload.subject,
        body_md: payload.body_md,
        thread_id: payload.thread_id,
        importance: payload.importance,
    };

    let message_id = lib_core::model::message::MessageBmc::create(&ctx, mm, msg_c).await?;

    Ok(Json(SendMessageResponse { message_id }).into_response())
}


// --- list_inbox ---
#[derive(Deserialize)]
pub struct ListInboxPayload {
    pub project_slug: String,
    pub agent_name: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    20
}

#[derive(Serialize)]
pub struct InboxMessage {
    pub id: i64,
    pub subject: String,
    pub sender_name: String,
    pub created_ts: chrono::NaiveDateTime,
}

pub async fn list_inbox(State(app_state): State<AppState>, Json(payload): Json<ListInboxPayload>) -> crate::error::Result<Response> {
    let ctx = Ctx::root_ctx();
    let mm = &app_state.mm;

    let project = lib_core::model::project::ProjectBmc::get_by_slug(&ctx, mm, &payload.project_slug).await?;
    let agent = lib_core::model::agent::AgentBmc::get_by_name(&ctx, mm, project.id, &payload.agent_name).await?;

    let messages = lib_core::model::message::MessageBmc::list_inbox_for_agent(&ctx, mm, project.id, agent.id, payload.limit).await?;

    let inbox_msgs: Vec<InboxMessage> = messages.into_iter().map(|msg| InboxMessage {
        id: msg.id,
        subject: msg.subject,
        sender_name: msg.sender_name,
        created_ts: msg.created_ts,
    }).collect();

    Ok(Json(inbox_msgs).into_response())
}

// --- list_all_projects ---
#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: i64,
    pub slug: String,
    pub human_key: String,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn list_all_projects(State(app_state): State<AppState>) -> crate::error::Result<Response> {
    let ctx = Ctx::root_ctx();
    let mm = &app_state.mm;

    let projects = lib_core::model::project::ProjectBmc::list_all(&ctx, mm).await?;

    let project_responses: Vec<ProjectResponse> = projects.into_iter().map(|p| ProjectResponse {
        id: p.id,
        slug: p.slug,
        human_key: p.human_key,
        created_at: p.created_at,
    }).collect();

    Ok(Json(project_responses).into_response())
}

// --- list_all_agents_for_project ---
// Keep for backwards compatibility with JSON body requests
#[derive(Deserialize)]
pub struct ListAgentsPayload {
    pub project_slug: String,
}

#[derive(Serialize)]
pub struct AgentResponse {
    pub id: i64,
    pub name: String,
    pub program: String,
    pub model: String,
    pub task_description: String,
    pub inception_ts: chrono::NaiveDateTime,
    pub last_active_ts: chrono::NaiveDateTime,
}

pub async fn list_all_agents_for_project(
    State(app_state): State<AppState>,
    Path(project_slug): Path<String>,
) -> crate::error::Result<Response> {
    let ctx = Ctx::root_ctx();
    let mm = &app_state.mm;

    let project = lib_core::model::project::ProjectBmc::get_by_slug(&ctx, mm, &project_slug).await?;
    let agents = lib_core::model::agent::AgentBmc::list_all_for_project(&ctx, mm, project.id).await?;

    let agent_responses: Vec<AgentResponse> = agents.into_iter().map(|a| AgentResponse {
        id: a.id,
        name: a.name,
        program: a.program,
        model: a.model,
        task_description: a.task_description,
        inception_ts: a.inception_ts,
        last_active_ts: a.last_active_ts,
    }).collect();

    Ok(Json(agent_responses).into_response())
}

// --- get_message ---
// Keep for backwards compatibility
#[derive(Deserialize)]
pub struct GetMessagePayload {
    pub message_id: i64,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: i64,
    pub project_id: i64,
    pub sender_id: i64,
    pub sender_name: String,
    pub thread_id: Option<String>,
    pub subject: String,
    pub body_md: String,
    pub importance: String,
    pub ack_required: bool,
    pub created_ts: chrono::NaiveDateTime,
    pub attachments: Vec<serde_json::Value>,
}

pub async fn get_message(
    State(app_state): State<AppState>,
    Path(message_id): Path<i64>,
) -> crate::error::Result<Response> {
    let ctx = Ctx::root_ctx();
    let mm = &app_state.mm;

    let message = lib_core::model::message::MessageBmc::get(&ctx, mm, message_id).await?;

    Ok(Json(MessageResponse {
        id: message.id,
        project_id: message.project_id,
        sender_id: message.sender_id,
        sender_name: message.sender_name,
        thread_id: message.thread_id,
        subject: message.subject,
        body_md: message.body_md,
        importance: message.importance,
        ack_required: message.ack_required,
        created_ts: message.created_ts,
        attachments: message.attachments,
    }).into_response())
}

// --- file_reservation_paths ---
#[derive(Deserialize)]
pub struct FileReservationPathsPayload {
    pub project_slug: String,
    pub agent_name: String,
    pub paths: Vec<String>,
    #[serde(default = "default_exclusive")]
    pub exclusive: bool,
    pub reason: Option<String>,
    pub ttl_seconds: Option<i64>,
}

fn default_exclusive() -> bool {
    true
}

#[derive(Serialize)]
pub struct FileReservationGranted {
    pub id: i64,
    pub path_pattern: String,
    pub exclusive: bool,
    pub reason: String,
    pub expires_ts: String,
}

#[derive(Serialize)]
pub struct FileReservationConflict {
    pub path_pattern: String,
    pub exclusive: bool,
    pub expires_ts: String,
    pub conflict_type: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct FileReservationPathsResponse {
    pub granted: Vec<FileReservationGranted>,
    pub conflicts: Vec<FileReservationConflict>,
}

pub async fn file_reservation_paths(State(app_state): State<AppState>, Json(payload): Json<FileReservationPathsPayload>) -> crate::error::Result<Response> {
    let ctx = Ctx::root_ctx();
    let mm = &app_state.mm;

    let project = lib_core::model::project::ProjectBmc::get_by_slug(&ctx, mm, &payload.project_slug).await?;
    let agent = lib_core::model::agent::AgentBmc::get_by_name(&ctx, mm, project.id, &payload.agent_name).await?;

    // 1. Get active reservations
    let active_reservations = FileReservationBmc::list_active_for_project(&ctx, mm, project.id).await?;

    let mut granted = Vec::new();
    let mut conflicts = Vec::new();

    let ttl = payload.ttl_seconds.unwrap_or(3600);
    let now = chrono::Utc::now().naive_utc();
    let expires_ts = now + chrono::Duration::seconds(ttl);

    for path in payload.paths {
        // Check conflicts
        // Simple overlap check for now (exact match)
        // TODO: Implement robust glob matching using globset
        for res in &active_reservations {
            if res.agent_id != agent.id {
                if res.exclusive || payload.exclusive {
                     if res.path_pattern == path {
                         conflicts.push(FileReservationConflict {
                             path_pattern: res.path_pattern.clone(),
                             exclusive: res.exclusive,
                             expires_ts: res.expires_ts.format("%Y-%m-%dT%H:%M:%S").to_string(),
                             conflict_type: "FILE_RESERVATION_CONFLICT".to_string(),
                             message: format!("Conflict with reservation held by agent ID {}", res.agent_id),
                         });
                         // We don't break, we collect all conflicts? Or just one per path?
                         // Python collects conflicts.
                     }
                }
            }
        }

        // Advisory model: always grant
        let fr_c = FileReservationForCreate {
            project_id: project.id,
            agent_id: agent.id,
            path_pattern: path.clone(),
            exclusive: payload.exclusive,
            reason: payload.reason.clone().unwrap_or_default(),
            expires_ts,
        };

        let id = FileReservationBmc::create(&ctx, mm, fr_c).await?;
        
        granted.push(FileReservationGranted {
            id,
            path_pattern: path,
            exclusive: payload.exclusive,
            reason: payload.reason.clone().unwrap_or_default(),
            expires_ts: expires_ts.format("%Y-%m-%dT%H:%M:%S").to_string(),
        });
    }

    Ok(Json(FileReservationPathsResponse {
        granted,
        conflicts,
    }).into_response())
}
