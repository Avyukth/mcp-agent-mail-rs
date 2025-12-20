//! Agent management tool implementations
//!
//! Handles agent registration, lookup, and profile management.

use lib_core::{
    ctx::Ctx,
    model::{
        ModelManager,
        agent::{AgentBmc, AgentForCreate, AgentProfileUpdate},
        file_reservation::FileReservationBmc,
    },
    utils::validation::{validate_agent_name, validate_project_key},
};
use rmcp::{ErrorData as McpError, model::CallToolResult, model::Content};
use std::sync::Arc;

use super::helpers;
use super::{GetAgentProfileParams, RegisterAgentParams, UpdateAgentProfileParams, WhoisParams};

/// Register an agent in a project.
pub async fn register_agent_impl(
    ctx: &Ctx,
    mm: &Arc<ModelManager>,
    params: RegisterAgentParams,
) -> Result<CallToolResult, McpError> {
    // Validate inputs
    validate_project_key(&params.project_slug).map_err(|e| {
        McpError::invalid_params(
            format!("{}", e),
            Some(serde_json::json!({ "details": e.context() })),
        )
    })?;

    validate_agent_name(&params.name).map_err(|e| {
        McpError::invalid_params(
            format!("{}", e),
            Some(serde_json::json!({ "details": e.context() })),
        )
    })?;

    // Get project
    let project = helpers::resolve_project(ctx, mm, &params.project_slug).await?;

    // Check if agent exists
    match AgentBmc::get_by_name(ctx, mm, project.id, &params.name).await {
        Ok(agent) => {
            let msg = format!(
                "Agent '{}' already exists (id: {}, program: {})",
                agent.name, agent.id, agent.program
            );
            Ok(CallToolResult::success(vec![Content::text(msg)]))
        }
        Err(_) => {
            let agent_c = AgentForCreate {
                project_id: project.id,
                name: params.name.clone(),
                program: params.program,
                model: params.model,
                task_description: params.task_description,
            };

            let id = AgentBmc::create(ctx, mm, agent_c)
                .await
                .map_err(|e| McpError::internal_error(e.to_string(), None))?;

            let msg = format!("Registered agent '{}' with id {}", params.name, id);
            Ok(CallToolResult::success(vec![Content::text(msg)]))
        }
    }
}

/// Get information about an agent.
pub async fn whois_impl(
    ctx: &Ctx,
    mm: &Arc<ModelManager>,
    params: WhoisParams,
) -> Result<CallToolResult, McpError> {
    let project = helpers::resolve_project(ctx, mm, &params.project_slug).await?;

    let agent = AgentBmc::get_by_name(ctx, mm, project.id, &params.agent_name)
        .await
        .map_err(|e| McpError::invalid_params(format!("Agent not found: {}", e), None))?;

    let output = format!(
        "Agent: {}\nID: {}\nProgram: {}\nModel: {}\nTask: {}\nContact Policy: {}\nAttachments Policy: {}",
        agent.name,
        agent.id,
        agent.program,
        agent.model,
        agent.task_description,
        agent.contact_policy,
        agent.attachments_policy
    );

    Ok(CallToolResult::success(vec![Content::text(output)]))
}

/// Update an agent's profile settings.
pub async fn update_agent_profile_impl(
    ctx: &Ctx,
    mm: &Arc<ModelManager>,
    params: UpdateAgentProfileParams,
) -> Result<CallToolResult, McpError> {
    let project = helpers::resolve_project(ctx, mm, &params.project_slug).await?;

    let agent = AgentBmc::get_by_name(ctx, mm, project.id, &params.agent_name)
        .await
        .map_err(|e| McpError::invalid_params(format!("Agent not found: {}", e), None))?;

    let update = AgentProfileUpdate {
        task_description: params.task_description,
        attachments_policy: params.attachments_policy,
        contact_policy: params.contact_policy,
    };

    AgentBmc::update_profile(ctx, mm, agent.id, update)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

    let msg = format!("Updated profile for agent '{}'", params.agent_name);
    Ok(CallToolResult::success(vec![Content::text(msg)]))
}

/// Get detailed profile information for an agent.
pub async fn get_agent_profile_impl(
    ctx: &Ctx,
    mm: &Arc<ModelManager>,
    params: GetAgentProfileParams,
) -> Result<CallToolResult, McpError> {
    let project = helpers::resolve_project(ctx, mm, &params.project_slug).await?;

    let agent = AgentBmc::get_by_name(ctx, mm, project.id, &params.agent_name)
        .await
        .map_err(|e| McpError::invalid_params(format!("Agent not found: {}", e), None))?;

    let sent_count = AgentBmc::count_messages_sent(ctx, mm, agent.id)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    let received_count = AgentBmc::count_messages_received(ctx, mm, agent.id)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

    let reservations = FileReservationBmc::list_active_for_project(ctx, mm, project.id)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    let active_reservations = reservations
        .iter()
        .filter(|r| r.agent_id == agent.id)
        .count();

    let output = format!(
        "Agent: {}\nID: {}\nProgram: {}\nModel: {}\nTask: {}\nContact Policy: {}\nAttachments Policy: {}\nMessages Sent: {}\nMessages Received: {}\nActive Reservations: {}\nInception: {}\nLast Active: {}",
        agent.name,
        agent.id,
        agent.program,
        agent.model,
        agent.task_description,
        agent.contact_policy,
        agent.attachments_policy,
        sent_count,
        received_count,
        active_reservations,
        agent.inception_ts,
        agent.last_active_ts
    );
    Ok(CallToolResult::success(vec![Content::text(output)]))
}

/// List all agents registered in a project.
pub async fn list_agents_impl(
    ctx: &Ctx,
    mm: &Arc<ModelManager>,
    project_slug: &str,
) -> Result<CallToolResult, McpError> {
    let project = helpers::resolve_project(ctx, mm, project_slug).await?;

    let agents = AgentBmc::list_all_for_project(ctx, mm, project.id)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

    let mut output = format!("Agents in '{}' ({}):\n\n", project_slug, agents.len());
    for a in &agents {
        output.push_str(&format!(
            "- {} (id: {}, program: {}, model: {})\n",
            a.name, a.id, a.program, a.model
        ));
    }

    Ok(CallToolResult::success(vec![Content::text(output)]))
}
