#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::command;
use reqwest::blocking::Client;
use serde_json::Value;
use base64::{encode, decode};
use std::str;

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Using HTMX & Rust is nice eh?", name)
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
    match client.get("https://r4.smarthealthit.org/Patient?_count=5").send() {
        Ok(resp) => match resp.json::<Value>() {
            Ok(json) => Ok(json.to_string()),
            Err(_) => Err("Failed to parse JSON response from server".into()),
        },
        Err(_) => Err("Failed to search patients".into()),
    }
}

#[command]
fn fetch_patient_details(patient_id: &str) -> Result<String, String> {
    let client = Client::new();
    let url = format!("https://r4.smarthealthit.org/Patient/{}", patient_id);
    match client.get(&url).send() {
        Ok(resp) => match resp.json::<Value>() {
            Ok(json) => Ok(json.to_string()),
            Err(_) => Err("Failed to parse JSON response from server".into()),
        },
        Err(_) => Err("Failed to fetch patient details".into()),
    }
}

#[command]
fn encrypt(message: &str) -> Result<String, String> {
    Ok(encode(message))
}

#[command]
fn decrypt(message: &str) -> Result<String, String> {
    match decode(message) {
        Ok(bytes) => match str::from_utf8(&bytes) {
            Ok(decoded_str) => Ok(decoded_str.to_string()),
            Err(_) => Err("Failed to convert decoded bytes to string".into()),
        },
        Err(_) => Err("Failed to decode base64 message".into()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, reset, scan_network, search_patients, fetch_patient_details, encrypt, decrypt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
