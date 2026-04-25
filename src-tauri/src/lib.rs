use tauri::{Emitter, Listener, Manager};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_single_instance::init as single_instance_init;

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
        .plugin(single_instance_init(|app, argv, _cwd| {
            // 当第二个实例启动时，将文件路径发送到主实例
            println!("Second instance launched with args: {:?}", argv);
            if let Some(file_path) = argv.get(1) {
                println!("Opening file from second instance: {}", file_path);
                if let Some(window) = app.get_webview_window("main") {
                    window.emit("open-file", file_path).ok();
                }
            }
        }))
        .invoke_handler(tauri::generate_handler![
            read_file,
            write_file,
            open_file_dialog,
            save_file_dialog
        ])
        .setup(|app| {
            // 监听文件拖放事件
            app.listen("tauri://file-drop", move |event| {
                let file_path = event.payload();
                println!("File dropped: {}", file_path);
            });
            
            // 在应用启动时处理命令行参数（首次启动）
            let handle = app.handle().clone();
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let file_path = args[1].clone();
                println!("Opening file from command line at startup: {}", file_path);
                // 延迟发送事件，确保窗口已经准备好
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    if let Some(window) = handle.get_webview_window("main") {
                        window.emit("open-file", file_path).ok();
                    }
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
