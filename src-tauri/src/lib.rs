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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, generate_thumbnails])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
