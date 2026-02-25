mod thumbnail;

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Emitter,
};
use thumbnail::ThumbnailService;

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

#[tauri::command]
async fn delete_all_thumbnails() -> Result<(), String> {
    tokio::task::spawn_blocking(|| thumbnail::delete_all())
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let mut menu_builder = MenuBuilder::new(handle);

            // macOS typically has an "App Name" menu as the first item
            #[cfg(target_os = "macos")]
            {
                let app_submenu = SubmenuBuilder::new(handle, "Media Viewer")
                    .about(None)
                    .separator()
                    .item(
                        &MenuItemBuilder::with_id("open-settings", "Settings...")
                            .accelerator("CmdOrCtrl+,")
                            .build(handle)?,
                    )
                    .separator()
                    .services()
                    .separator()
                    .hide()
                    .hide_others()
                    .show_all()
                    .separator()
                    .quit()
                    .build()?;

                menu_builder = menu_builder.item(&app_submenu);
            }

            // Other generic submenus
            let file_submenu = SubmenuBuilder::new(handle, "File").close_window().build()?;

            let window_submenu = SubmenuBuilder::new(handle, "Window")
                .minimize()
                .maximize()
                .separator()
                .close_window()
                .build()?;

            menu_builder = menu_builder.items(&[&file_submenu, &window_submenu]);

            #[cfg(not(target_os = "macos"))]
            {
                // For Windows/Linux, typically settings is under File or Edit
                let edit_submenu = SubmenuBuilder::new(handle, "Edit")
                    .item(
                        &MenuItemBuilder::with_id("open-settings", "Settings...")
                            .accelerator("CmdOrCtrl+,")
                            .build(handle)?,
                    )
                    .build()?;

                menu_builder = menu_builder.item(&edit_submenu);
            }

            let menu = menu_builder.build()?;
            app.set_menu(menu)?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            if event.id() == "open-settings" {
                let _ = app.emit("open-settings", ());
            }
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            generate_thumbnails,
            cleanup_thumbnails_for_dir,
            cleanup_orphan_thumbnails,
            delete_all_thumbnails
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
