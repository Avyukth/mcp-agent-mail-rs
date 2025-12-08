use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub product_uid: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductProjectLink {
    pub id: i64,
    pub product_id: i64,
    pub project_id: i64,
    pub created_at: NaiveDateTime,
}
