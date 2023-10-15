// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{dictionary::get_def, language_parsing::parse_text};

// use tauri::Window;
mod dictionary;
mod language_parsing;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_text, get_def])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, serde::Serialize)]
struct SentenceEvent {
    sent: String,
}

// #[tauri::command]
// fn parse_text(_window: Window, sent: &str) -> String {
//     format!("lol {sent}")
// }
