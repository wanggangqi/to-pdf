use sqlx::SqlitePool;
use tauri::{Manager, Emitter};
use crate::db::*;
use crate::models::{Task, CreateTask, ProviderConfig};
use crate::services::{Translator, PdfGenerator, document_processor::DocumentProcessor};

#[tauri::command]
pub async fn list_tasks(app_handle: tauri::AppHandle) -> Result<Vec<Task>, String> {
    let pool = app_handle.state::<SqlitePool>();
    list_tasks_db(&*pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_task(
    app_handle: tauri::AppHandle,
    document_id: String,
) -> Result<Task, String> {
    let pool = app_handle.state::<SqlitePool>();
    let pool_clone = pool.inner().clone();

    // 创建任务
    let task = create_task_db(&*pool, CreateTask { document_id: document_id.clone() })
        .await
        .map_err(|e| e.to_string())?;

    // 启动后台处理
    let app_handle_clone = app_handle.clone();
    let task_id = task.id.clone();

    tokio::spawn(async move {
        if let Err(e) = process_task(app_handle_clone, pool_clone, task_id, document_id).await {
            eprintln!("Task processing error: {}", e);
        }
    });

    Ok(task)
}

#[tauri::command]
pub async fn delete_task(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let pool = app_handle.state::<SqlitePool>();
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn process_task(
    app_handle: tauri::AppHandle,
    pool: SqlitePool,
    task_id: String,
    document_id: String,
) -> anyhow::Result<()> {
    // 更新状态为 processing
    update_task_status_db(&pool, &task_id, "processing", 0).await?;
    app_handle.emit("task-progress", &serde_json::json!({
        "taskId": task_id,
        "status": "processing",
        "progress": 0
    }))?;

    // 获取文档
    let docs = list_documents_db(&pool).await?;
    let doc = docs.iter().find(|d| d.id == document_id)
        .ok_or_else(|| anyhow::anyhow!("Document not found"))?;

    // 提取文本
    let text = DocumentProcessor::extract_text(&doc.path, &doc.doc_type)?;
    let paragraphs = DocumentProcessor::split_into_paragraphs(&text);

    if paragraphs.is_empty() {
        fail_task_db(&pool, &task_id, "文档无文本内容").await?;
        return Ok(());
    }

    // 从配置文件加载提供商配置
    let provider = match load_active_provider(&app_handle) {
        Ok(p) => p,
        Err(e) => {
            fail_task_db(&pool, &task_id, &e.to_string()).await?;
            app_handle.emit("task-progress", &serde_json::json!({
                "taskId": task_id,
                "status": "failed",
                "progress": 0,
                "error": e.to_string()
            }))?;
            return Err(e);
        }
    };

    // 翻译
    let translator = Translator::new(provider);
    let translated_results = translator.translate_paragraphs(&paragraphs, 4).await?;

    // 组装结果
    let translated: Vec<(String, String)> = paragraphs.iter()
        .zip(translated_results.iter())
        .map(|(cn, en)| (cn.clone(), en.clone()))
        .collect();

    // 更新进度到 95%
    update_task_status_db(&pool, &task_id, "processing", 95).await?;
    app_handle.emit("task-progress", &serde_json::json!({
        "taskId": task_id,
        "status": "processing",
        "progress": 95
    }))?;

    // 生成 PDF
    let output_dir = app_handle.path().app_data_dir()?;
    let output_path = output_dir.join(format!("output_{}.pdf", task_id));
    PdfGenerator::generate_bilingual_pdf(&output_path.to_string_lossy(), &translated)?;

    // 完成
    complete_task_db(&pool, &task_id, &output_path.to_string_lossy()).await?;
    app_handle.emit("task-progress", &serde_json::json!({
        "taskId": task_id,
        "status": "completed",
        "progress": 100
    }))?;

    Ok(())
}

/// 从配置文件加载活动的提供商
fn load_active_provider(app_handle: &tauri::AppHandle) -> anyhow::Result<ProviderConfig> {
    let config_dir = app_handle.path().app_config_dir()?;
    let config_path = config_dir.join("providers.json");

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let providers: Vec<ProviderConfig> = serde_json::from_str(&content)?;

        if let Some(provider) = providers.into_iter().find(|p| p.is_active && !p.api_key.is_empty()) {
            return Ok(provider);
        }
    }

    anyhow::bail!("No active provider configured. Please configure API key in Settings.")
}