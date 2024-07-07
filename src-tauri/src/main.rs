#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from the HTMX-TAURI Stack!", name)
}

//resets the String to blank
#[command]
fn reset() -> String {
    "".to_string()
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
