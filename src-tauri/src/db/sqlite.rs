use tauri::{AppHandle, Manager};
use anyhow::Result;

/// Initialize the SQLite database
pub async fn init_db(app: &AppHandle) -> Result<()> {
    let app_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("doctranslate.db");
    let db_url = format!("sqlite:{}", db_path.display());

    // TODO: Initialize database connection pool
    println!("Database would be initialized at: {}", db_url);

    Ok(())
}

/// Get the database path
pub fn get_db_path(app: &AppHandle) -> Result<std::path::PathBuf> {
    let app_dir = app.path().app_data_dir()?;
    Ok(app_dir.join("doctranslate.db"))
}