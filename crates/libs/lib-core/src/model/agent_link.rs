use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentLink {
    pub id: i64,
    pub a_project_id: i64,
    pub a_agent_id: i64,
    pub b_project_id: i64,
    pub b_agent_id: i64,
    pub status: String,
    pub reason: String,
    pub created_ts: NaiveDateTime,
    pub updated_ts: NaiveDateTime,
    pub expires_ts: Option<NaiveDateTime>,
}
