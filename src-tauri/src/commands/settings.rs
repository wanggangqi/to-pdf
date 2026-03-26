use tauri::Manager;
use crate::models::ProviderConfig;
use crate::services::ai_client::test_connection;

#[tauri::command]
pub async fn get_providers(app_handle: tauri::AppHandle) -> Vec<ProviderConfig> {
    // 尝试从配置文件加载已保存的配置
    if let Ok(config_dir) = app_handle.path().app_config_dir() {
        let config_path = config_dir.join("providers.json");
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(saved) = serde_json::from_str::<Vec<ProviderConfig>>(&content) {
                    // 合并保存的配置和预设（以保存的为主）
                    let presets = ProviderConfig::presets();
                    let mut result = Vec::new();

                    for preset in presets {
                        if let Some(saved_provider) = saved.iter().find(|p| p.id == preset.id) {
                            // 使用保存的配置
                            result.push(saved_provider.clone());
                        } else {
                            // 使用预设
                            result.push(preset);
                        }
                    }

                    // 添加预设中没有的新配置
                    for saved_provider in saved {
                        if !result.iter().any(|p| p.id == saved_provider.id) {
                            result.push(saved_provider);
                        }
                    }

                    return result;
                }
            }
        }
    }

    // 返回预设
    ProviderConfig::presets()
}

#[tauri::command]
pub async fn save_provider(
    app_handle: tauri::AppHandle,
    provider: ProviderConfig,
) -> Result<(), String> {
    let config_dir = app_handle.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    let config_path = config_dir.join("providers.json");
    let mut providers: Vec<ProviderConfig> = if config_path.exists() {
        let content = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        ProviderConfig::presets()
    };

    // 如果提供了 API Key，自动启用该提供商
    let mut provider = provider;
    if !provider.api_key.is_empty() {
        provider.is_active = true;
    }

    // 更新或添加 provider
    if let Some(existing) = providers.iter_mut().find(|p| p.id == provider.id) {
        *existing = provider;
    } else {
        providers.push(provider);
    }

    let content = serde_json::to_string_pretty(&providers).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, content).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn test_provider(provider: ProviderConfig) -> Result<String, String> {
    test_connection(&provider).await.map_err(|e| e.to_string())
}