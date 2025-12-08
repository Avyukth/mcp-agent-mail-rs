use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSiblingSuggestion {
    pub id: i64,
    pub project_a_id: i64,
    pub project_b_id: i64,
    pub score: f64,
    pub status: String,
    pub rationale: String,
    pub created_ts: NaiveDateTime,
    pub evaluated_ts: NaiveDateTime,
    pub confirmed_ts: Option<NaiveDateTime>,
    pub dismissed_ts: Option<NaiveDateTime>,
}
