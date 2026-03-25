use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub doc_type: String, // "word" | "pdf"
    pub size: i64,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub vectorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocument {
    pub name: String,
    #[serde(rename = "type")]
    pub doc_type: String,
    pub size: i64,
    pub path: String,
}