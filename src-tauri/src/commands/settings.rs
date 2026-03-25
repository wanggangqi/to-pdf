use tauri::Manager;
use crate::models::ProviderConfig;
use crate::services::ai_client::test_connection;

#[tauri::command]
pub async fn get_providers() -> Vec<ProviderConfig> {
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