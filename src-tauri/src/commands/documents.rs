use sqlx::SqlitePool;
use tauri::Manager;
use crate::db::{list_documents_db, create_document_db, delete_document_db};
use crate::models::{Document, CreateDocument};

#[tauri::command]
pub async fn list_documents(app_handle: tauri::AppHandle) -> Result<Vec<Document>, String> {
    let pool = app_handle.state::<SqlitePool>();
    list_documents_db(&pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn upload_document(
    app_handle: tauri::AppHandle,
    file_path: String,
) -> Result<Document, String> {
    let pool = app_handle.state::<SqlitePool>();

    // Get file info
    let path = std::path::Path::new(&file_path);
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let doc_type = match extension.as_str() {
        "doc" | "docx" => "word",
        "pdf" => "pdf",
        _ => return Err("Unsupported file type".to_string()),
    };

    let metadata = std::fs::metadata(&file_path).map_err(|e| e.to_string())?;
    let size = metadata.len() as i64;

    // Copy file to application data directory
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let docs_dir = app_dir.join("documents");
    std::fs::create_dir_all(&docs_dir).map_err(|e| e.to_string())?;

    // Generate unique filename to avoid conflicts
    let id = uuid::Uuid::new_v4().to_string();
    let dest_name = format!("{}_{}", id, name);
    let dest_path = docs_dir.join(&dest_name);
    std::fs::copy(&file_path, &dest_path).map_err(|e| e.to_string())?;

    let doc = CreateDocument {
        name,
        doc_type: doc_type.to_string(),
        size,
        path: dest_path.to_string_lossy().to_string(),
    };

    create_document_db(&pool, doc).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_document(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<(), String> {
    let pool = app_handle.state::<SqlitePool>();

    // Get document path and delete file
    let docs = list_documents_db(&pool).await.map_err(|e| e.to_string())?;
    if let Some(doc) = docs.iter().find(|d| d.id == id) {
        if std::path::Path::new(&doc.path).exists() {
            std::fs::remove_file(&doc.path).map_err(|e| e.to_string())?;
        }
    }

    delete_document_db(&pool, &id).await.map_err(|e| e.to_string())
}