// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{add_to_anki::add_to_anki, dictionary::get_defs, language_parsing::parse_text};
use ankiconnect::get_anki_card_statuses;
use serde::{Deserialize, Serialize};
use shared::{SakinyjeResult, Settings};
use std::{collections::HashMap, fs};
use tauri::{async_runtime::block_on, GlobalWindowEvent, Manager, State, WindowEvent};

mod add_to_anki;
mod ankiconnect;
mod dictionary;
mod language_parsing;

struct SakinyjeState(tauri::async_runtime::Mutex<SharedInfo>);

struct SharedInfo {
    settings: Settings,
    to_save: ToSave,
}

#[derive(Serialize, Deserialize, Default)]
struct ToSave {
    words: HashMap<String, WordInfo>,
    cached_defs: HashMap<String, Vec<SakinyjeResult<String>>>,
}

impl Default for SharedInfo {
    fn default() -> Self {
        let saved_state_file = dirs::data_dir().unwrap().join("sakinyje_saved_data.toml");
        let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");

        let mut to_save: ToSave = fs::read_to_string(saved_state_file)
            .map(|v| toml::from_str(&v).unwrap())
            .unwrap_or_default();

        let settings: Settings = fs::read_to_string(config_file)
            .map(|v| toml::from_str(&v).unwrap()) // TODO: some sort of error handling when invalid
            // toml is used
            .unwrap_or_default();

        if let Some(ankiparsers) = &settings.anki_parser {
            for (deck, note_parser) in ankiparsers {
                block_on(get_anki_card_statuses(
                    &deck,
                    note_parser,
                    &mut to_save.words,
                ))
                .unwrap();
                // TODO: handle error
            }
        }
        Self { to_save, settings }
    }
}

#[derive(Serialize, Deserialize)]
struct WordInfo {
    rating: u8,
    can_modify: bool,
}

fn main() {
    tauri::Builder::default()
        .manage(SakinyjeState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            parse_text,
            get_defs,
            get_settings,
            add_to_anki,
            write_settings,
        ])
        .on_window_event(handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_window_event(event: GlobalWindowEvent) {
    block_on(async move {
        match event.event() {
            &WindowEvent::Destroyed => {
                let saved_state_file = dirs::data_dir().unwrap().join("sakinyje_saved_data.toml");
                let state: State<'_, SakinyjeState> = event.window().state();
                let locked_state = state.0.lock().await;
                let conts =
                    toml::to_string(&locked_state.to_save).expect("Error serializing to toml");
                fs::write(saved_state_file, conts).expect("error writing to file");
            }
            _ => (),
        }
    })
}

#[tauri::command]
async fn get_settings(state: State<'_, SakinyjeState>) -> Result<Settings, String> {
    let state = state.0.lock().await;
    Ok(state.settings.clone())
}

#[tauri::command]
async fn write_settings(state: State<'_, SakinyjeState>, settings: Settings) -> Result<(), String> {
    let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");
    let conts = toml::to_string_pretty(&settings).unwrap();

    let mut state = state.0.lock().await;
    state.settings = settings;

    fs::write(config_file, conts).map_err(|e| e.to_string())
}
