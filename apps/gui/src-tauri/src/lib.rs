use lunio_client::FileEntry;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command(async)]
async fn search(query: String) -> Result<Vec<FileEntry>, String> {
    lunio_client::search(&query, Some(50))
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}