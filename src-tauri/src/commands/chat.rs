use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::{Manager, Emitter};
use crate::db::*;
use crate::models::{Document, ProviderConfig};
use crate::services::{ai_client, embedding, document_processor::DocumentProcessor};
use crate::vector::VectorStore;

/// 从配置文件加载活动的提供商
fn load_active_provider(app_handle: &tauri::AppHandle) -> Result<ProviderConfig, String> {
    let config_dir = app_handle.path().app_config_dir().map_err(|e| e.to_string())?;
    let config_path = config_dir.join("providers.json");

    println!("[DEBUG] Loading provider config from: {:?}", config_path);

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        println!("[DEBUG] Config content: {}", content);

        let providers: Vec<ProviderConfig> = serde_json::from_str(&content)
            .map_err(|e| e.to_string())?;

        println!("[DEBUG] Loaded {} providers", providers.len());
        for p in &providers {
            println!("[DEBUG] Provider: id={}, isActive={}, apiKey={}",
                p.id, p.is_active,
                if p.api_key.is_empty() { "empty" } else { "set" });
        }

        if let Some(provider) = providers.into_iter().find(|p| p.is_active && !p.api_key.is_empty()) {
            println!("[DEBUG] Active provider found: {}", provider.id);
            return Ok(provider);
        } else {
            println!("[DEBUG] No active provider with API key found");
        }
    } else {
        println!("[DEBUG] Config file not found");
    }

    Err("No active provider configured. Please configure API key in Settings.".to_string())
}

#[tauri::command]
pub async fn chat(
    app_handle: tauri::AppHandle,
    message: String,
    document_ids: Vec<String>,
) -> Result<String, String> {
    let pool = app_handle.state::<SqlitePool>();

    // 从配置文件加载活动提供商
    let provider = load_active_provider(&app_handle)?;

    // 有选择文档 -> RAG
    if !document_ids.is_empty() {
        let docs = list_documents_db(&*pool).await.map_err(|e| e.to_string())?;
        let selected: Vec<&Document> = docs.iter()
            .filter(|d| document_ids.contains(&d.id) && d.vectorized)
            .collect();

        if selected.is_empty() {
            return Ok("所选文档未向量化或不存在，请先向量化文档。".to_string());
        }

        // 获取查询向量
        let query_embedding = embedding::get_embedding(&provider, &message)
            .await
            .map_err(|e| e.to_string())?;

        // 向量搜索
        let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
        let db_path = app_dir.join("vectors.db");
        let vector_store = VectorStore::new(&db_path.to_string_lossy());

        let results = vector_store
            .search(&query_embedding, 5)
            .await
            .map_err(|e| e.to_string())?;

        if results.is_empty() {
            return Ok("在所选文档中未找到相关信息。".to_string());
        }

        // 构建上下文（提取 text 字段）
        let context_text = results
            .iter()
            .map(|(_, text, _)| text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        let system_prompt = format!(
            "你是一个文档问答助手。根据以下文档内容回答用户问题。\
             \n\n文档内容：\n{}\n\n\
             如果文档中没有相关信息，请直接说'文档中未找到相关信息'。",
            context_text
        );

        let messages = vec![
            ai_client::Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            ai_client::Message {
                role: "user".to_string(),
                content: message,
            },
        ];

        ai_client::chat(&provider, messages).await.map_err(|e| e.to_string())
    } else {
        // 普通聊天
        let messages = vec![
            ai_client::Message {
                role: "user".to_string(),
                content: message,
            },
        ];

        ai_client::chat(&provider, messages).await.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn vectorize_document(
    app_handle: tauri::AppHandle,
    document_id: String,
) -> Result<(), String> {
    println!("[DEBUG] vectorize_document called: {}", document_id);

    let pool = app_handle.state::<SqlitePool>();

    let docs = list_documents_db(&*pool).await.map_err(|e| e.to_string())?;
    let doc = docs.iter().find(|d| d.id == document_id)
        .ok_or_else(|| "Document not found".to_string())?;

    println!("[DEBUG] Document found: {}, type: {}", doc.name, doc.doc_type);

    // 提取文本
    let text = DocumentProcessor::extract_text(&doc.path, &doc.doc_type)
        .map_err(|e| e.to_string())?;
    let paragraphs = DocumentProcessor::split_into_paragraphs(&text);

    println!("[DEBUG] Extracted {} paragraphs", paragraphs.len());

    if paragraphs.is_empty() {
        return Err("文档无文本内容".to_string());
    }

    // 从配置文件加载提供商
    println!("[DEBUG] Loading active provider...");
    let provider = load_active_provider(&app_handle)?;

    println!("[DEBUG] Using provider: {}, embedding model: {}", provider.id, provider.embedding_model);

    // 发送开始事件
    let _ = app_handle.emit("vectorize-progress", &serde_json::json!({
        "documentId": document_id,
        "status": "started",
        "total": paragraphs.len()
    }));

    // 获取嵌入（带进度回调和取消检查）
    let app_handle_for_callback = app_handle.clone();
    let document_id_for_callback = document_id.clone();
    let pool_for_cancel = Arc::new(pool.inner().clone());
    let document_id_for_cancel = document_id.clone();

    let on_progress = Box::new(move |batch: usize, total: usize| {
        let _ = app_handle_for_callback.emit("vectorize-progress", &serde_json::json!({
            "documentId": document_id_for_callback,
            "status": "progress",
            "batch": batch,
            "totalBatches": total
        }));
    });

    // 取消检查：检查文档是否还存在于数据库中
    let should_cancel = Box::new(move || {
        let pool = pool_for_cancel.clone();
        let doc_id = document_id_for_cancel.clone();

        // 使用 tokio block_in_place 在异步上下文中执行同步阻塞操作
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                match list_documents_db(&*pool).await {
                    Ok(docs) => docs.iter().any(|d| d.id == doc_id),
                    Err(_) => false,
                }
            })
        })
    });

    let embeddings = embedding::get_embeddings_with_callback(
        &provider,
        &paragraphs,
        Some(on_progress),
        Some(should_cancel),
    ).await.map_err(|e| {
        if e.to_string().contains("cancelled") {
            "文档已删除，向量化已取消".to_string()
        } else {
            e.to_string()
        }
    })?;

    println!("[DEBUG] Got {} embeddings", embeddings.len());

    // 存储向量
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("vectors.db");
    println!("[DEBUG] Vector store path: {:?}", db_path);

    let vector_store = VectorStore::new(&db_path.to_string_lossy());

    // 生成段落 ID
    let ids: Vec<String> = paragraphs
        .iter()
        .enumerate()
        .map(|(i, _)| format!("{}-{}", document_id, i))
        .collect();

    vector_store
        .insert(&ids, &document_id, &paragraphs, &embeddings)
        .await
        .map_err(|e| e.to_string())?;

    println!("[DEBUG] Vectors inserted successfully");

    // 更新文档状态
    update_vectorized_db(&*pool, &document_id, true)
        .await
        .map_err(|e| e.to_string())?;

    // 发送完成事件
    let _ = app_handle.emit("vectorize-progress", &serde_json::json!({
        "documentId": document_id,
        "status": "completed"
    }));

    println!("[DEBUG] Document marked as vectorized");

    Ok(())
}