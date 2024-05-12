// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
    add_to_anki::add_to_anki,
    ankiconnect::{get_all_deck_names, get_all_note_names, get_note_field_names, remove_deck},
    dictionary::get_defs,
    language_parsing::parse_text,
};
use ankiconnect::get_anki_card_statuses;
use chrono::{DateTime, Utc};
use commands::run_command;
use pyo3::PyObject;
use serde::{Deserialize, Serialize};
use shared::{SakinyjeResult, Settings};
use spacy_parsing::get_spacy_model;
use std::{collections::HashMap, fs};
use tauri::{async_runtime::block_on, GlobalWindowEvent, Manager, State, WindowEvent};

mod add_to_anki;
mod ankiconnect;
mod commands;
mod dictionary;
mod language_parsing;

struct SakinyjeState(tauri::async_runtime::Mutex<SharedInfo>);

struct SharedInfo {
    settings: Settings,
    to_save: ToSave,
    model: PyObject,
}

#[derive(Serialize, Deserialize, Default)]
struct ToSave {
    words: HashMap<String, WordInfo>,
    cached_defs: HashMap<String, Vec<SakinyjeResult<String>>>,
    last_launched: DateTime<Utc>,
    decks_checked: Vec<String>,
}

impl Default for SharedInfo {
    fn default() -> Self {
        let saved_state_file = dirs::data_dir().unwrap().join("sakinyje_saved_data.toml");
        let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");

        let mut to_save: ToSave = fs::read_to_string(saved_state_file)
            .map(|v| toml::from_str(&v).unwrap_or_default())
            .unwrap_or_default();

        let mut settings: Settings = fs::read_to_string(config_file)
            .map(|v| toml::from_str(&v).unwrap()) // TODO: some sort of error handling when invalid
            // toml is used
            .unwrap_or_default();

        let new_time = Utc::now();
        let days_passed = new_time
            .signed_duration_since(to_save.last_launched)
            .num_days()
            + 2;

        for (deck, note_parser) in &mut settings.anki_parser {
            block_on(get_anki_card_statuses(
                deck,
                &note_parser.0,
                &mut to_save.words,
                days_passed,
                !to_save.decks_checked.contains(deck),
            ))
            .unwrap();
            to_save.decks_checked.push(deck.clone());
        }
        update_words_known(
            &settings.frequency_list,
            settings.words_known_by_freq,
            &mut to_save.words,
        );

        if let Some(cmds) = &settings.to_run {
            for cmd in cmds {
                _ = run_command(cmd);
            }
        }

        let model = get_spacy_model(&settings.model).unwrap(); // TODO: model

        to_save.last_launched = new_time;
        Self {
            to_save,
            settings,
            model,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct WordInfo {
    rating: u8,
    method: Method,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
enum Method {
    FromAnki,
    FromSeen,
    Specified,
    FromFrequency,
}

fn main() {
    tauri::Builder::default()
        .manage(SakinyjeState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            parse_text,
            get_defs,
            get_settings,
            get_dark_mode,
            add_to_anki,
            write_settings,
            get_all_deck_names,
            get_all_note_names,
            get_note_field_names,
            update_word_knowledge,
            remove_deck,
        ])
        .on_window_event(handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_window_event(event: GlobalWindowEvent) {
    block_on(async move {
        #[allow(clippy::single_match)] // Will probably be expanded in the future
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

fn update_words_known(
    file_name: &str,
    words_known: usize,
    original_words: &mut HashMap<String, WordInfo>,
) {
    println!("updating words known");
    if let Ok(contents) = fs::read_to_string(file_name) {
        original_words.retain(|_, v| v.method != Method::FromFrequency);
        for word in contents.lines().take(words_known) {
            println!("{word}");
            original_words.insert(
                word.to_owned(),
                WordInfo {
                    rating: 5,
                    method: Method::FromFrequency,
                },
            );
        }
    }
}

#[tauri::command]
async fn get_settings(state: State<'_, SakinyjeState>) -> Result<Settings, String> {
    let state = state.0.lock().await;
    Ok(state.settings.clone())
}

#[tauri::command]
async fn get_dark_mode(state: State<'_, SakinyjeState>) -> Result<bool, String> {
    let state = state.0.lock().await;
    Ok(state.settings.dark_mode)
}

#[tauri::command]
async fn write_settings(state: State<'_, SakinyjeState>, settings: Settings) -> Result<(), String> {
    let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");
    let conts = toml::to_string_pretty(&settings).unwrap();

    let mut state = state.0.lock().await;
    state.settings = settings;

    fs::write(config_file, conts).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_word_knowledge(
    state: State<'_, SakinyjeState>,
    word: &str,
    rating: u8,
    modifiable: bool,
) -> Result<(), String> {
    let mut state = state.0.lock().await;
    let word_knowledge = state.to_save.words.get_mut(word).unwrap();
    word_knowledge.rating = rating;
    word_knowledge.method = if modifiable {
        Method::FromAnki
    } else {
        Method::Specified
    };
    Ok(())
}
