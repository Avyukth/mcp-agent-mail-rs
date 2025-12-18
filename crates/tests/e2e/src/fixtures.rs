//! Test fixtures for creating test data

use serde::Deserialize;
use uuid::Uuid;

/// Test fixtures for E2E tests
pub struct TestFixtures;

impl TestFixtures {
    /// Generate a unique human-readable project name for testing
    /// Returns a display name like "Test Project abc123"
    pub fn unique_project_name() -> String {
        format!("Test Project {}", &Uuid::new_v4().to_string()[..8])
    }

    /// Generate a unique project slug for testing (used after project creation)
    pub fn unique_project_slug() -> String {
        format!("test-project-{}", &Uuid::new_v4().to_string()[..8])
    }

    /// Generate a unique agent name for testing
    pub fn unique_agent_name() -> String {
        format!("test-agent-{}", &Uuid::new_v4().to_string()[..8])
    }

    /// Create project payload
    /// Note: API uses `human_key` (display name), not `project_slug`
    pub fn project_payload(human_key: &str) -> serde_json::Value {
        serde_json::json!({
            "human_key": human_key
        })
    }

    /// Create agent registration payload
    /// Note: API uses `name` field, not `agent_name`
    pub fn agent_payload(project_slug: &str, name: &str) -> serde_json::Value {
        serde_json::json!({
            "project_slug": project_slug,
            "name": name,
            "program": "test-runner",
            "model": "test-model"
        })
    }

    /// Create message payload
    pub fn message_payload(
        project_slug: &str,
        sender: &str,
        recipients: &[&str],
        subject: &str,
        body: &str,
    ) -> serde_json::Value {
        serde_json::json!({
            "project_slug": project_slug,
            "sender_name": sender,
            "recipient_names": recipients,
            "subject": subject,
            "body_md": body
        })
    }
}

/// Response from ensure_project endpoint
#[derive(Debug, Deserialize)]
pub struct ProjectResponse {
    pub id: i64,
    pub slug: String,
    pub human_key: String,
}

/// Response from register_agent endpoint
#[derive(Debug, Deserialize)]
pub struct AgentResponse {
    pub id: i64,
    pub name: String,
    pub project_id: i64,
    pub program: String,
    pub model: String,
    pub task_description: String,
    pub inception_ts: String,
    pub last_active_ts: String,
}

/// Response from send_message endpoint
#[derive(Debug, Deserialize)]
pub struct MessageResponse {
    pub id: i64,
    pub thread_id: String,
}
