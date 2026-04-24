use tauri::{Emitter, Listener};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
async fn read_file(path: String) -> Result<String, String> {
    tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| format!("读取文件失败: {}", e))
}

#[tauri::command]
async fn write_file(path: String, content: String) -> Result<(), String> {
    tokio::fs::write(&path, content)
        .await
        .map_err(|e| format!("写入文件失败: {}", e))
}

#[tauri::command]
async fn open_file_dialog(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    let file_path = app_handle
        .dialog()
        .file()
        .add_filter("Markdown Files", &["md", "markdown"])
        .blocking_pick_file();
    
    Ok(file_path.and_then(|p| p.as_path().map(|path| path.to_string_lossy().to_string())))
}

#[tauri::command]
async fn save_file_dialog(app_handle: tauri::AppHandle, default_name: Option<String>) -> Result<Option<String>, String> {
    let file_path = app_handle
        .dialog()
        .file()
        .add_filter("Markdown Files", &["md", "markdown"])
        .set_file_name(default_name.unwrap_or_else(|| "untitled".to_string()))
        .blocking_save_file();
    
    Ok(file_path.and_then(|p| p.as_path().map(|path| path.to_string_lossy().to_string())))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            read_file,
            write_file,
            open_file_dialog,
            save_file_dialog
        ])
        .setup(|app| {
            // 监听文件打开事件
            app.listen("tauri://file-drop", move |event| {
                let file_path = event.payload();
                println!("File dropped: {}", file_path);
                // 这里可以触发前端打开文件的逻辑
            });
            Ok(())
        })
        .on_page_load(|window, _payload| {
            // 检查是否有命令行参数传入的文件路径
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let file_path = &args[1];
                window.emit("open-file", file_path).ok();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
