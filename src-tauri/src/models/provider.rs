use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub embedding_model: String,
    pub is_active: bool,
}

impl ProviderConfig {
    pub fn presets() -> Vec<ProviderConfig> {
        vec![
            ProviderConfig {
                id: "deepseek".to_string(),
                name: "DeepSeek".to_string(),
                api_key: String::new(),
                base_url: "https://api.deepseek.com".to_string(),
                model: "deepseek-chat".to_string(),
                embedding_model: "deepseek-embedding".to_string(),
                is_active: false,
            },
            ProviderConfig {
                id: "moonshot".to_string(),
                name: "Moonshot".to_string(),
                api_key: String::new(),
                base_url: "https://api.moonshot.cn".to_string(),
                model: "moonshot-v1-8k".to_string(),
                embedding_model: "moonshot-embedding-v1".to_string(),
                is_active: false,
            },
            ProviderConfig {
                id: "zhipu".to_string(),
                name: "智谱".to_string(),
                api_key: String::new(),
                base_url: "https://open.bigmodel.cn/api/paas/v4".to_string(),
                model: "glm-4".to_string(),
                embedding_model: "embedding-3".to_string(),
                is_active: false,
            },
            ProviderConfig {
                id: "bailian".to_string(),
                name: "百炼".to_string(),
                api_key: String::new(),
                base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1".to_string(),
                model: "qwen-turbo".to_string(),
                embedding_model: "text-embedding-v3".to_string(),
                is_active: false,
            },
        ]
    }
}