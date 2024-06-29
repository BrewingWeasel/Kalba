// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
    add_to_anki::add_to_anki,
    ankiconnect::{get_all_deck_names, get_all_note_names, get_note_field_names, remove_deck},
    dictionary::{get_defs, DictionaryInfo},
    language_parsing::parse_text,
    new_language_template::new_language_from_template,
};
use ankiconnect::get_anki_card_statuses;
use chrono::{DateTime, Utc};
use commands::run_command;
use pyo3::PyObject;
use serde::{Deserialize, Serialize};
use shared::{SakinyjeResult, Settings};
use spacy_parsing::get_spacy_model;
use std::{collections::HashMap, fs, sync::Arc};
use tauri::{async_runtime::block_on, GlobalWindowEvent, Manager, State, WindowEvent};

mod add_to_anki;
mod ankiconnect;
mod commands;
mod dictionary;
mod language_parsing;
mod new_language_template;

#[derive(Debug, thiserror::Error)]
enum SakinyjeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
    #[error("No operating system {0} directory was found")]
    MissingDir(String),
    #[error("Anki is not available. This may be because it is not open or ankiconnect is not installed.")]
    AnkiNotAvailable,
    #[error("Unable to download language details from github: {0}")]
    LanugageDetailsDownloading(#[from] reqwest::Error),
    #[error("The selected card has handler that fits its model")]
    NoModelHandler,
    #[error("Ankiconnect return an error: {0}")]
    AnkiConnectError(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for SakinyjeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

struct SakinyjeState(tauri::async_runtime::Mutex<SharedInfo>);

struct SharedInfo {
    settings: Settings,
    to_save: ToSave,
    model: Option<PyObject>,
    current_language: Option<String>,
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
}

#[derive(Serialize, Deserialize, Default)]
struct ToSave {
    last_launched: DateTime<Utc>,
    last_language: Option<String>,
    decks_checked: Vec<String>,
    language_specific: HashMap<String, LanguageSpecficToSave>,
}

#[derive(Serialize, Deserialize, Default)]
struct LanguageSpecficToSave {
    words: HashMap<String, WordInfo>,
    cached_defs: HashMap<String, Vec<SakinyjeResult<String>>>,
}

impl Default for SharedInfo {
    fn default() -> Self {
        let saved_state_file = dirs::data_dir().unwrap().join("sakinyje_saved_data.toml");
        let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");

        let mut to_save: ToSave = fs::read_to_string(saved_state_file)
            .map(|v| toml::from_str(&v).unwrap_or_default())
            .unwrap_or_default();

        let settings: Settings = fs::read_to_string(config_file)
            .map(|v| toml::from_str(&v).unwrap()) // TODO: some sort of error handling when invalid
            // toml is used
            .unwrap_or_default();

        let new_time = Utc::now();
        let days_passed = new_time
            .signed_duration_since(to_save.last_launched)
            .num_days()
            + 2;

        for (language_name, language) in &settings.languages {
            let to_save_language = to_save
                .language_specific
                .entry(language_name.to_owned())
                .or_default();
            for (deck, note_parser) in &language.anki_parser {
                block_on(get_anki_card_statuses(
                    deck,
                    &note_parser.0,
                    &mut to_save_language.words,
                    days_passed,
                    // If the deck has not been added, it means this is the first time it is being
                    // checked, so we should check every card and not just the ones recently
                    // updated
                    !to_save.decks_checked.contains(deck),
                ))
                .unwrap();
                to_save.decks_checked.push(deck.to_owned());
            }
            update_words_known(
                &language.frequency_list,
                language.words_known_by_freq,
                &mut to_save_language.words,
            );
        }

        if let Some(cmds) = &settings.to_run {
            for cmd in cmds {
                _ = run_command(cmd);
            }
        }

        let model = None; // TODO: model

        to_save.last_launched = new_time;
        let current_language = to_save.last_language.clone();
        Self {
            to_save,
            settings,
            model,
            current_language,
            dict_info: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct WordInfo {
    rating: i8,
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
            get_rating,
            get_language,
            set_language,
            get_languages,
            new_language_from_template,
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
                let mut locked_state = state.0.lock().await;
                locked_state.to_save.last_language = locked_state.current_language.clone();
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
async fn get_rating(lemma: String, state: State<'_, SakinyjeState>) -> Result<i8, String> {
    let mut state = state.0.lock().await;
    let language = state
        .current_language
        .clone()
        .expect("need a language selected to be able to set rating");
    Ok(state
        .to_save
        .language_specific
        .get_mut(&language)
        .expect("language should exist")
        .words
        .entry(lemma)
        .or_insert(WordInfo {
            rating: 0,
            method: Method::FromSeen,
        })
        .rating)
}

#[tauri::command]
async fn get_languages(state: State<'_, SakinyjeState>) -> Result<Vec<String>, String> {
    let state = state.0.lock().await;
    Ok(state.settings.languages.keys().cloned().collect())
}

#[tauri::command]
async fn get_language(state: State<'_, SakinyjeState>) -> Result<Option<String>, String> {
    let state = state.0.lock().await;
    Ok(state.current_language.to_owned())
}

#[tauri::command]
async fn set_language(state: State<'_, SakinyjeState>, language: String) -> Result<(), String> {
    let mut state = state.0.lock().await;
    state.current_language = Some(language);
    Ok(())
}

#[tauri::command]
async fn write_settings(
    state: State<'_, SakinyjeState>,
    settings: Settings,
) -> Result<(), SakinyjeError> {
    let config_file = dirs::config_dir()
        .ok_or(SakinyjeError::MissingDir("config".to_string()))?
        .join("sakinyje.toml");
    let conts = toml::to_string_pretty(&settings)?;

    let mut state = state.0.lock().await;
    state.settings = settings;

    fs::write(config_file, conts)?;
    Ok(())
}

#[tauri::command]
async fn update_word_knowledge(
    state: State<'_, SakinyjeState>,
    word: &str,
    rating: i8,
    modifiable: bool,
) -> Result<(), String> {
    let mut state = state.0.lock().await;
    let language = state
        .current_language
        .clone()
        .expect("current language should already be chosen");
    let word_knowledge = state
        .to_save
        .language_specific
        .get_mut(&language)
        .expect("current language should already have content to save")
        .words
        .get_mut(word)
        .unwrap();
    word_knowledge.rating = rating;
    word_knowledge.method = if modifiable {
        Method::FromAnki
    } else {
        Method::Specified
    };
    Ok(())
}
