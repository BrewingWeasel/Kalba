// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{add_to_anki::add_to_anki, dictionary::get_def, language_parsing::parse_text};
use shared::Settings;
use std::fs;

mod add_to_anki;
mod dictionary;
mod language_parsing;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            parse_text,
            get_def,
            get_settings,
            add_to_anki,
            write_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_settings() -> Settings {
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
            css: None,
        },
    }
}

#[tauri::command]
fn write_settings(settings: Settings) -> Result<(), String> {
    let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");
    let conts = toml::to_string_pretty(&settings).unwrap();
    fs::write(config_file, conts).map_err(|e| e.to_string())
}
