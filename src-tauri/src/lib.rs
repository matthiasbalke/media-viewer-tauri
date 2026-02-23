mod thumbnail;

use thumbnail::ThumbnailService;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn generate_thumbnails(
    dir: String,
    session_id: u64,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    ThumbnailService::generate_for_dir(dir, session_id, app_handle).await
}

#[tauri::command]
async fn cleanup_thumbnails_for_dir(dir: String) -> Result<u32, String> {
    tokio::task::spawn_blocking(move || thumbnail::cleanup_for_prefix(&dir))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
async fn cleanup_orphan_thumbnails() -> Result<u32, String> {
    tokio::task::spawn_blocking(|| thumbnail::cleanup_orphans())
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_thumbnails,
            cleanup_thumbnails_for_dir,
            cleanup_orphan_thumbnails
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
