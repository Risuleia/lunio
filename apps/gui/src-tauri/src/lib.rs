use tauri::Manager;

mod client;
mod commands;
mod system;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            }

            #[cfg(target_os = "windows")]
            use window_vibrancy::apply_acrylic;
            apply_acrylic(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::cmd_connect,
            commands::cmd_search,
            commands::cmd_list_dir,
            commands::cmd_request_thumbnail,
            commands::cmd_get_thumbnail,
            commands::cmd_shutdown,
            system::get_sidebar_entries
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}