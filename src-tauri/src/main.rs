#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::command;
use std::env;

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've connected HTMX to RUST. Nice huh?", name)
}

#[command]
fn reset() -> String {
    "".to_string()
}

#[command]
fn scan_network(ip: &str) -> Result<String, String> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {}", current_dir.display());

    let script_path = current_dir.join("src").join("network_scanner.py");

    let output = Command::new("python")
        .arg(script_path)
        .arg(ip)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, reset, scan_network])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}