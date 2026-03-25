use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tauri::Manager;
use crate::models::{Document, CreateDocument};

pub async fn init_db(app_handle: &tauri::AppHandle) -> Result<()> {
    let app_dir = app_handle.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("doctranslate.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // 创建表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            doc_type TEXT NOT NULL,
            size INTEGER NOT NULL,
            path TEXT NOT NULL,
            created_at TEXT NOT NULL,
            vectorized INTEGER NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // 存储 pool 到 app state
    app_handle.manage(pool);

    Ok(())
}

pub async fn list_documents_db(pool: &SqlitePool) -> Result<Vec<Document>> {
    let rows = sqlx::query_as::<_, Document>(
        "SELECT id, name, doc_type as doc_type, size, path, created_at, vectorized FROM documents ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn create_document_db(pool: &SqlitePool, doc: CreateDocument) -> Result<Document> {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now();

    sqlx::query(
        "INSERT INTO documents (id, name, doc_type, size, path, created_at, vectorized) VALUES (?, ?, ?, ?, ?, ?, 0)"
    )
    .bind(&id)
    .bind(&doc.name)
    .bind(&doc.doc_type)
    .bind(doc.size)
    .bind(&doc.path)
    .bind(created_at.to_rfc3339())
    .execute(pool)
    .await?;

    Ok(Document {
        id,
        name: doc.name,
        doc_type: doc.doc_type,
        size: doc.size,
        path: doc.path,
        created_at,
        vectorized: false,
    })
}

pub async fn delete_document_db(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM documents WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_vectorized_db(pool: &SqlitePool, id: &str, vectorized: bool) -> Result<()> {
    sqlx::query("UPDATE documents SET vectorized = ? WHERE id = ?")
        .bind(vectorized)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}