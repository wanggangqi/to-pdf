use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::ProviderConfig;

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: String,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

/// Get embedding for a single text
pub async fn get_embedding(provider: &ProviderConfig, text: &str) -> Result<Vec<f32>> {
    let client = Client::new();
    let url = format!("{}/embeddings", provider.base_url);

    let request = EmbeddingRequest {
        model: provider.embedding_model.clone(),
        input: text.to_string(),
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

    embedding_response
        .data
        .first()
        .map(|d| d.embedding.clone())
        .ok_or_else(|| anyhow!("No embedding returned"))
}

/// Get embeddings for multiple texts
pub async fn get_embeddings(provider: &ProviderConfig, texts: &[String]) -> Result<Vec<Vec<f32>>> {
    let mut results = Vec::new();
    for text in texts {
        let embedding = get_embedding(provider, text).await?;
        results.push(embedding);
    }
    Ok(results)
}