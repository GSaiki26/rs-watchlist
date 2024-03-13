// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_config_file(file_path: &str) {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create(file_path).expect("Couldn't create file.");
    file.write_all(b"{}").expect("Couldn't write to file.");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_config_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
