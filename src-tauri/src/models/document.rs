use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub filename: String,
    pub file_type: FileType,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
    pub status: DocumentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    Pdf,
    Docx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentStatus {
    Pending,
    Processing,
    Completed,
    Error,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Pdf => write!(f, "pdf"),
            FileType::Docx => write!(f, "docx"),
        }
    }
}

impl std::fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentStatus::Pending => write!(f, "pending"),
            DocumentStatus::Processing => write!(f, "processing"),
            DocumentStatus::Completed => write!(f, "completed"),
            DocumentStatus::Error => write!(f, "error"),
        }
    }
}