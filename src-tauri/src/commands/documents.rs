use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub filename: String,
    pub file_type: String,
    pub file_size: i64,
    pub created_at: String,
    pub status: String,
}

#[tauri::command]
pub async fn list_documents() -> Result<Vec<Document>, String> {
    // TODO: Implement loading from database
    Ok(vec![])
}

#[tauri::command]
pub async fn upload_document(file_path: String) -> Result<Document, String> {
    // TODO: Implement document upload and parsing
    println!("Uploading document: {}", file_path);
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn delete_document(id: String) -> Result<(), String> {
    // TODO: Implement document deletion
    println!("Deleting document: {}", id);
    Ok(())
}