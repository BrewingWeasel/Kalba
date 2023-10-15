// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{dictionary::get_def, language_parsing::parse_text};
use shared::Settings;
use std::fs;
use tauri::Window;

// use tauri::Window;
mod dictionary;
mod language_parsing;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_text, get_def, get_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_settings(_window: Window) -> Settings {
    let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");
    match fs::read_to_string(config_file) {
        Ok(v) => toml::from_str(&v).unwrap(),
        Err(_) => Settings {
            deck: String::from("Default"),
            note_type: String::from("Basic"),
            note_fields: String::from(
                "Front:$sent
Back:$def",
            ),
            model: String::new(),
            dicts: Vec::new(),
            to_remove: None,
        },
    }
}
