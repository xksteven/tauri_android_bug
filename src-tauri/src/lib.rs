// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::fs;
use std::path::PathBuf;
use base64::{engine::general_purpose, Engine as _}; // Add base64 dependency


#[tauri::command]
fn open_file(path: String) -> Result<String, String> {
    // println!("Opening file at path: {}", path);

    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        // println!("Error: File does not exist at path: {}", path);
        return Err(format!("File does not exist at path: {}", path));
    }

    if !path_buf.is_file() {
        // println!("Error: Path is not a file: {}", path);
        return Err(format!("Path is not a file: {}", path));
    }

    match fs::read(&path_buf) {
        Ok(data) => {
            // println!("File read successfully. Size: {} bytes", data.len());
            let encoded = general_purpose::STANDARD.encode(data); // Base64 encode the data
            Ok(encoded)
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            Err(e.to_string())
        }
    }
}

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![open_file])
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
