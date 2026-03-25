// Embedding service module
// TODO: Implement embedding service

pub struct EmbeddingService {
    client: crate::services::ai_client::AiClient,
}

impl EmbeddingService {
    pub fn new(client: crate::services::ai_client::AiClient) -> Self {
        Self { client }
    }

    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>, anyhow::Error> {
        self.client.embed(text).await
    }

    pub async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, anyhow::Error> {
        let mut embeddings = Vec::new();
        for text in texts {
            embeddings.push(self.embed_text(text).await?);
        }
        Ok(embeddings)
    }
}