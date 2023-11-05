// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {

    match list_files_in_folder(name) {
        Ok(result) => format!("Path: {}, Files: {}!", name, result.join("\n")),
        Err(err) => format!("Error: {}", err),
    }
}

fn list_files_in_folder(folder_path: &str) -> Result<Vec<String>, io::Error> {
    let mut file_list = Vec::new();

    let entries = fs::read_dir(folder_path)?;
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        file_list.push(file_name.to_string_lossy().to_string());
    }

    Ok(file_list)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
