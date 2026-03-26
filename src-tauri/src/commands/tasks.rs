use sqlx::SqlitePool;
use std::sync::Arc;
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

#[tauri::command]
pub async fn open_task_output(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let pool = app_handle.state::<SqlitePool>();

    // 获取任务信息
    let tasks = list_tasks_db(&*pool).await.map_err(|e| e.to_string())?;
    let task = tasks.iter().find(|t| t.id == id)
        .ok_or_else(|| "Task not found".to_string())?;

    let output_path = task.output_path.as_ref()
        .ok_or_else(|| "Task has no output file".to_string())?;

    // 使用系统默认程序打开文件
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", output_path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(output_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(output_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

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
        let error_msg = "文档无文本内容";
        fail_task_db(&pool, &task_id, error_msg).await?;
        app_handle.emit("task-progress", &serde_json::json!({
            "taskId": task_id,
            "status": "failed",
            "progress": 0,
            "error": error_msg
        }))?;
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

    // 翻译（带进度）
    let app_handle_for_progress = app_handle.clone();
    let task_id_for_progress = task_id.clone();
    let total_paragraphs = paragraphs.len();

    // 发送翻译开始事件
    app_handle.emit("task-progress", &serde_json::json!({
        "taskId": task_id,
        "status": "translating",
        "phase": "translation",
        "progress": 5,
        "total": total_paragraphs
    }))?;

    let on_progress = Arc::new(move |completed: usize, total: usize| {
        // 计算翻译进度（占 5%-85%）
        let progress = 5 + ((completed as f32 / total as f32) * 80.0) as i32;
        let _ = app_handle_for_progress.emit("task-progress", &serde_json::json!({
            "taskId": task_id_for_progress,
            "status": "translating",
            "phase": "translation",
            "progress": progress,
            "completed": completed,
            "total": total
        }));
    });

    // 使用更高的并发数（10）
    let translated_results = match Translator::translate_paragraphs_with_progress(
        provider,
        &paragraphs,
        10,
        Some(on_progress)
    ).await {
        Ok(results) => results,
        Err(e) => {
            let error_msg = e.to_string();
            fail_task_db(&pool, &task_id, &error_msg).await?;
            app_handle.emit("task-progress", &serde_json::json!({
                "taskId": task_id,
                "status": "failed",
                "progress": 50,
                "error": error_msg
            }))?;
            return Err(e);
        }
    };

    // 组装结果
    let translated: Vec<(String, String)> = paragraphs.iter()
        .zip(translated_results.iter())
        .map(|(cn, en)| (cn.clone(), en.clone()))
        .collect();

    // 更新进度到 90%
    update_task_status_db(&pool, &task_id, "processing", 90).await?;
    app_handle.emit("task-progress", &serde_json::json!({
        "taskId": task_id,
        "status": "processing",
        "progress": 90
    }))?;

    // 生成 PDF
    let output_dir = app_handle.path().app_data_dir()?;
    let output_path = output_dir.join(format!("output_{}.pdf", task_id));
    if let Err(e) = PdfGenerator::generate_bilingual_pdf(&output_path.to_string_lossy(), &translated) {
        let error_msg = e.to_string();
        fail_task_db(&pool, &task_id, &error_msg).await?;
        app_handle.emit("task-progress", &serde_json::json!({
            "taskId": task_id,
            "status": "failed",
            "progress": 95,
            "error": error_msg
        }))?;
        return Err(e);
    }

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