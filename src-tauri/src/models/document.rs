use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub doc_type: String, // "word" | "pdf"
    pub size: i64,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub vectorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocument {
    pub name: String,
    pub doc_type: String,
    pub size: i64,
    pub path: String,
}