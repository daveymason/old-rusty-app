#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::command;
use reqwest::blocking::Client;
use serde_json::Value;

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
    let output = Command::new("python")
        .arg("src/network_scanner.py")
        .arg(ip)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

#[command]
fn search_patients() -> Result<String, String> {
    let client = Client::new();
    let response = client.get("https://r4.smarthealthit.org/Patient?_count=5")
        .send();

    match response {
        Ok(resp) => {
            let json: Value = resp.json().unwrap();
            Ok(json.to_string())
        }
        Err(_) => Err("Failed to search patients".into()),
    }
}

#[command]
fn fetch_patient_details(patient_id: &str) -> Result<String, String> {
    let client = Client::new();
    let url = format!("https://r4.smarthealthit.org/Patient/{}", patient_id);
    let response = client.get(&url)
        .send();

    match response {
        Ok(resp) => {
            let json: Value = resp.json().unwrap();
            Ok(json.to_string())
        }
        Err(_) => Err("Failed to fetch patient details".into()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, reset, scan_network, search_patients, fetch_patient_details])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}