mod commands;
mod db;
mod models;
mod services;
mod vector;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::block_on(async {
                if let Err(e) = db::init_db(&app_handle).await {
                    eprintln!("Failed to initialize database: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Settings commands
            commands::settings::get_providers,
            commands::settings::save_provider,
            commands::settings::test_provider,
            // Document commands
            commands::documents::list_documents,
            commands::documents::upload_document,
            commands::documents::delete_document,
            // Task commands
            commands::tasks::list_tasks,
            commands::tasks::create_task,
            commands::tasks::delete_task,
            commands::tasks::open_task_output,
            // Chat commands
            commands::chat::chat,
            commands::chat::vectorize_document,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}