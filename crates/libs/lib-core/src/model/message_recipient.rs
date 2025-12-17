use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRecipient {
    pub message_id: i64,
    pub agent_id: i64,
    pub recipient_type: String, // e.g., "to", "cc", "bcc"
    pub read_ts: Option<NaiveDateTime>,
    pub ack_ts: Option<NaiveDateTime>,
}
