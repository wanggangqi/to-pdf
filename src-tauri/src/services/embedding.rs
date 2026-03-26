use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::ProviderConfig;

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
    index: usize,
}

/// Batch size for embedding API requests
/// Note: 百炼 API 限制每批最多 10 个文本
pub const EMBEDDING_BATCH_SIZE: usize = 10;

/// Progress callback for embedding batches
pub type ProgressCallback = Box<dyn Fn(usize, usize) + Send + Sync>;

/// Cancel check callback - returns false to cancel
pub type CancelCheck = Box<dyn Fn() -> bool + Send + Sync>;

/// Get embeddings for multiple texts with progress callback and cancel support
pub async fn get_embeddings_with_callback(
    provider: &ProviderConfig,
    texts: &[String],
    on_progress: Option<ProgressCallback>,
    should_cancel: Option<CancelCheck>,
) -> Result<Vec<Vec<f32>>> {
    if texts.is_empty() {
        return Ok(Vec::new());
    }

    let client = Client::new();
    let url = format!("{}/embeddings", provider.base_url);

    let total_batches = (texts.len() + EMBEDDING_BATCH_SIZE - 1) / EMBEDDING_BATCH_SIZE;
    let mut all_embeddings = vec![None; texts.len()];

    for batch_idx in 0..total_batches {
        // 检查是否取消
        if let Some(ref check) = should_cancel {
            if !check() {
                println!("[DEBUG] Embedding cancelled at batch {}/{}", batch_idx + 1, total_batches);
                return Err(anyhow!("Operation cancelled"));
            }
        }

        let start = batch_idx * EMBEDDING_BATCH_SIZE;
        let end = std::cmp::min(start + EMBEDDING_BATCH_SIZE, texts.len());
        let batch: Vec<String> = texts[start..end].to_vec();

        println!("[DEBUG] Embedding batch {}/{} (texts {}-{})",
            batch_idx + 1, total_batches, start + 1, end);

        // 调用进度回调
        if let Some(ref callback) = on_progress {
            callback(batch_idx + 1, total_batches);
        }

        // 百炼 v4 需要指定维度，默认使用 1024
        let dimension = if provider.embedding_model.contains("v4") {
            Some(1024)
        } else {
            None
        };

        let request = EmbeddingRequest {
            model: provider.embedding_model.clone(),
            input: batch.clone(),
            dimension,
        };

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", provider.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Embedding API error: {}", error_text));
        }

        let embedding_response: EmbeddingResponse = response.json().await?;

        // API 返回的 data 按 index 排序，确保顺序正确
        let mut batch_embeddings: Vec<(usize, Vec<f32>)> = embedding_response
            .data
            .into_iter()
            .map(|d| (d.index, d.embedding))
            .collect();
        batch_embeddings.sort_by_key(|(i, _)| *i);

        for (local_idx, embedding) in batch_embeddings {
            all_embeddings[start + local_idx] = Some(embedding);
        }
    }

    // 确保所有 embedding 都已获取
    let results: Vec<Vec<f32>> = all_embeddings
        .into_iter()
        .enumerate()
        .map(|(i, e)| e.ok_or_else(|| anyhow!("Missing embedding at index {}", i)))
        .collect::<Result<Vec<Vec<f32>>>>()?;

    // 打印向量维度
    if !results.is_empty() {
        println!("[DEBUG] Vector dimension: {}", results[0].len());
    }
    println!("[DEBUG] Total embeddings retrieved: {}", results.len());
    Ok(results)
}

/// Get embeddings for multiple texts (simple version without progress callback)
pub async fn get_embeddings(provider: &ProviderConfig, texts: &[String]) -> Result<Vec<Vec<f32>>> {
    get_embeddings_with_callback(provider, texts, None, None).await
}

/// Get embedding for a single text
pub async fn get_embedding(provider: &ProviderConfig, text: &str) -> Result<Vec<f32>> {
    let texts: Vec<String> = vec![text.to_string()];
    let embeddings = get_embeddings(provider, &texts).await?;
    embeddings.into_iter().next().ok_or_else(|| anyhow!("No embedding returned"))
}