use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub enabled: bool,
}

#[tauri::command]
pub async fn get_providers() -> Result<Vec<ProviderConfig>, String> {
    // TODO: Implement loading from storage
    Ok(vec![
        ProviderConfig {
            id: "deepseek".to_string(),
            name: "DeepSeek".to_string(),
            api_key: String::new(),
            base_url: "https://api.deepseek.com".to_string(),
            model: "deepseek-chat".to_string(),
            enabled: false,
        },
        ProviderConfig {
            id: "moonshot".to_string(),
            name: "Moonshot".to_string(),
            api_key: String::new(),
            base_url: "https://api.moonshot.cn".to_string(),
            model: "moonshot-v1-8k".to_string(),
            enabled: false,
        },
        ProviderConfig {
            id: "zhipu".to_string(),
            name: "智谱".to_string(),
            api_key: String::new(),
            base_url: "https://open.bigmodel.cn".to_string(),
            model: "glm-4".to_string(),
            enabled: false,
        },
        ProviderConfig {
            id: "bailian".to_string(),
            name: "百炼".to_string(),
            api_key: String::new(),
            base_url: "https://dashscope.aliyuncs.com".to_string(),
            model: "qwen-turbo".to_string(),
            enabled: false,
        },
    ])
}

#[tauri::command]
pub async fn save_provider(provider: ProviderConfig) -> Result<(), String> {
    // TODO: Implement saving to storage
    println!("Saving provider: {:?}", provider);
    Ok(())
}

#[tauri::command]
pub async fn test_provider(provider_id: String) -> Result<bool, String> {
    // TODO: Implement testing connection
    println!("Testing provider: {}", provider_id);
    Ok(true)
}