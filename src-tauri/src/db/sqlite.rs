use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tauri::Manager;
use crate::models::{Document, CreateDocument, Task, CreateTask};

pub async fn init_db(app_handle: &tauri::AppHandle) -> Result<()> {
    let app_dir = app_handle.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("doctranslate.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // 创建 documents 表
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

    // 创建 tasks 表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            document_id TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            progress INTEGER NOT NULL DEFAULT 0,
            output_path TEXT,
            error TEXT,
            created_at TEXT NOT NULL,
            completed_at TEXT,
            FOREIGN KEY (document_id) REFERENCES documents(id)
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

pub async fn list_tasks_db(pool: &SqlitePool) -> Result<Vec<Task>> {
    let rows = sqlx::query_as::<_, Task>(
        "SELECT id, document_id, status, progress, output_path, error, created_at, completed_at FROM tasks ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn create_task_db(pool: &SqlitePool, task: CreateTask) -> Result<Task> {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now();

    sqlx::query(
        "INSERT INTO tasks (id, document_id, status, progress, created_at) VALUES (?, ?, 'pending', 0, ?)"
    )
    .bind(&id)
    .bind(&task.document_id)
    .bind(created_at.to_rfc3339())
    .execute(pool)
    .await?;

    Ok(Task {
        id,
        document_id: task.document_id,
        status: "pending".to_string(),
        progress: 0,
        output_path: None,
        error: None,
        created_at,
        completed_at: None,
    })
}

pub async fn update_task_status_db(pool: &SqlitePool, id: &str, status: &str, progress: i32) -> Result<()> {
    sqlx::query("UPDATE tasks SET status = ?, progress = ? WHERE id = ?")
        .bind(status)
        .bind(progress)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn complete_task_db(pool: &SqlitePool, id: &str, output_path: &str) -> Result<()> {
    let completed_at = chrono::Utc::now().to_rfc3339();

    sqlx::query("UPDATE tasks SET status = 'completed', progress = 100, output_path = ?, completed_at = ? WHERE id = ?")
        .bind(output_path)
        .bind(completed_at)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn fail_task_db(pool: &SqlitePool, id: &str, error: &str) -> Result<()> {
    sqlx::query("UPDATE tasks SET status = 'failed', error = ? WHERE id = ?")
        .bind(error)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}