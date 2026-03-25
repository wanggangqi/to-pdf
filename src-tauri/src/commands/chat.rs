use sqlx::SqlitePool;
use tauri::Manager;
use crate::db::*;
use crate::models::{Document, ProviderConfig};
use crate::services::{ai_client, embedding, document_processor::DocumentProcessor};
use crate::vector::VectorStore;

/// 从配置文件加载活动的提供商
fn load_active_provider(app_handle: &tauri::AppHandle) -> Result<ProviderConfig, String> {
    let config_dir = app_handle.path().app_config_dir().map_err(|e| e.to_string())?;
    let config_path = config_dir.join("providers.json");

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        let providers: Vec<ProviderConfig> = serde_json::from_str(&content)
            .map_err(|e| e.to_string())?;

        if let Some(provider) = providers.into_iter().find(|p| p.is_active && !p.api_key.is_empty()) {
            return Ok(provider);
        }
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

        // 构建上下文
        let context_text = results.join("\n\n---\n\n");

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
    let pool = app_handle.state::<SqlitePool>();

    let docs = list_documents_db(&*pool).await.map_err(|e| e.to_string())?;
    let doc = docs.iter().find(|d| d.id == document_id)
        .ok_or_else(|| "Document not found".to_string())?;

    // 提取文本
    let text = DocumentProcessor::extract_text(&doc.path, &doc.doc_type)
        .map_err(|e| e.to_string())?;
    let paragraphs = DocumentProcessor::split_into_paragraphs(&text);

    if paragraphs.is_empty() {
        return Err("文档无文本内容".to_string());
    }

    // 从配置文件加载提供商
    let provider = load_active_provider(&app_handle)?;

    // 获取嵌入
    let embeddings = embedding::get_embeddings(&provider, &paragraphs)
        .await
        .map_err(|e| e.to_string())?;

    // 存储向量
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("vectors.db");
    let vector_store = VectorStore::new(&db_path.to_string_lossy());

    vector_store
        .insert(&document_id, embeddings)
        .await
        .map_err(|e| e.to_string())?;

    // 更新文档状态
    update_vectorized_db(&*pool, &document_id, true)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}