use pulldown_cmark::{html, Options, Parser};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
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
        .add_filter("Markdown & Text Files", &["md", "markdown", "txt"])
        .blocking_pick_file();
    
    Ok(file_path.and_then(|p| p.as_path().map(|path| path.to_string_lossy().to_string())))
}

#[tauri::command]
async fn markdown_to_html(markdown: String) -> Result<String, String> {
    let parser = Parser::new_ext(&markdown, Options::all());
    let mut html_output = String::with_capacity(markdown.len() * 2);
    html::push_html(&mut html_output, parser);
    Ok(html_output)
}

#[tauri::command]
async fn save_file_dialog(app_handle: tauri::AppHandle, default_name: Option<String>) -> Result<Option<String>, String> {
    let file_path = app_handle
        .dialog()
        .file()
        .add_filter("Markdown & Text Files", &["md", "markdown", "txt"])
        .set_file_name(default_name.unwrap_or_else(|| "untitled".to_string()))
        .blocking_save_file();
    
    Ok(file_path.and_then(|p| p.as_path().map(|path| path.to_string_lossy().to_string())))
}

#[tauri::command]
async fn open_file_in_new_window(path: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("{}", e))?
        .as_millis();
    let label = format!("file-{}", timestamp);
    
    let title = std::path::Path::new(&path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());
    
    let encoded = path
        .replace('%', "%25")
        .replace('#', "%23")
        .replace('?', "%3F")
        .replace('&', "%26");
    let url = format!("index.html?file={}", encoded);
    
    WebviewWindowBuilder::new(
        &app_handle,
        &label,
        WebviewUrl::App(url.into())
    )
    .title(format!("Markdown Editor - {}", title))
    .build()
    .map_err(|e| format!("Failed to create window: {}", e))?;
    
    Ok(())
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
            save_file_dialog,
            markdown_to_html,
            open_file_in_new_window
        ])
        .setup(|app| {
            // 在应用启动时处理命令行参数
            let handle = app.handle().clone();
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let file_path = args[1].clone();
                println!("Opening file from command line: {}", file_path);
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
