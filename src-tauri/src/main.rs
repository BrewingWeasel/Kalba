#![allow(clippy::literal_string_with_formatting_args)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
    add_to_anki::{add_to_anki, get_export_variables},
    ankiconnect::{get_all_deck_names, get_all_note_names, get_note_field_names, remove_deck},
    dictionary::{get_definition_on_demand, get_defs, DictionaryInfo},
    language_parsing::{get_url_contents, parse_text, parse_url, read_file, start_stanza},
    new_language_template::new_language_from_template,
    setup_stanza::{check_stanza_installed, setup_stanza, uninstall_stanza},
};
use ankiconnect::get_anki_card_statuses;
use chrono::{DateTime, TimeDelta, Utc};
use commands::run_command;
use new_language_template::use_language_template;
use serde::{Deserialize, Serialize};
use shared::{Definition, LanguageSettings, Settings, StartingSettings, ToasterPayload};
use simple_logger::SimpleLogger;
use spyglys_integration::{format_spyglys, get_spyglys_functions};
use stats::{get_words_added, get_words_known_at_levels, time_spent};
use std::{collections::HashMap, fs, io::BufReader, process, sync::Arc, time::Duration};
use tauri::{async_runtime::block_on, Emitter, Manager, State, Window, WindowEvent};
use tokio::sync::MutexGuard;

mod add_to_anki;
mod ankiconnect;
mod commands;
mod dictionary;
mod language_parsing;
mod new_language_template;
mod setup_stanza;
mod spyglys_integration;
mod stats;

#[derive(Debug, thiserror::Error)]
enum KalbaError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
    #[error("Error reading {0}: {1}")]
    TomlDe(String, toml::de::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    Spyglys(#[from] spyglys::SpyglysError),
    #[error(transparent)]
    SpyglysRuntime(#[from] spyglys::interpreter::RuntimeError),
    #[error(transparent)]
    HtmlRewrite(#[from] lol_html::errors::RewritingError),
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Stardict(#[from] stardict::error::Error),
    #[error(transparent)]
    CacheDecode(#[from] rmp_serde::decode::Error),
    #[error("No operating system {0} directory was found")]
    MissingDir(String),
    #[error("Anki is not available. This may be because it is not open or ankiconnect is not installed.")]
    AnkiNotAvailable,
    #[error("Unable to download language details from github: {0}")]
    LanguageDetailsDownloading(#[from] reqwest::Error),
    #[error("Ankiconnect return an error: {0}")]
    AnkiConnectError(String),
    #[error("Only wrote {1} bytes of `{0}`")]
    IncorrectWrite(String, usize),
    #[error("Unable to find a version of python installed on the system. It may not be installed or in the PATH.")]
    PythonNotFound,
    #[error("Pip install failed. Maybe you ran out of space?")]
    PipInstallFailed,
    #[error("Python version ({0}) does not match. Version 3.8 or later is required for stanza")]
    WrongPythonVersion(String),
    #[error("File {0} is not a valid file type. See the docs for more.")]
    InvalidFileType(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for KalbaError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

struct KalbaState(tauri::async_runtime::Mutex<SharedInfo>);

struct SharedInfo {
    settings: Settings,
    to_save: ToSave,
    language_parser: Option<LanguageParser>,
    current_language: Option<String>,
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
    errors: Vec<KalbaError>,
    can_save: bool,
    language_cached_data: HashMap<String, CachedData>,
    in_reader: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct CachedData {
    definitions: HashMap<String, HashMap<String, Definition>>,
}

struct LanguageParser {
    stdin: process::ChildStdin,
    stdout: BufReader<process::ChildStdout>,
}

fn default_version() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

#[derive(Serialize, Deserialize, Default)]
struct ToSave {
    installing_stanza: bool,
    last_launched: DateTime<Utc>,
    last_language: Option<String>,
    decks_checked: Vec<String>,
    language_specific: HashMap<String, LanguageSpecificToSave>,
    sessions: Vec<(DateTime<Utc>, Duration)>,
    #[serde(default = "default_version")]
    kalba_version: String,
}

#[derive(Serialize, Deserialize, Default)]
struct LanguageSpecificToSave {
    words: HashMap<String, WordInfo>,
    previous_file: Option<String>,
    lemmas_to_replace: HashMap<String, String>,
    previous_amount: usize,
    words_seen: Vec<(DateTime<Utc>, usize)>,
    added_to_anki: Vec<(DateTime<Utc>, String)>,
}

impl Default for SharedInfo {
    fn default() -> Self {
        let mut errors = Vec::new();
        let mut can_save = true;

        let mut to_save: ToSave = match dirs::data_dir()
            .ok_or_else(|| KalbaError::MissingDir(String::from("data")))
            .and_then(|saved_state_file| {
                fs::read_to_string(saved_state_file.join("kalba").join("saved_data.toml"))
                    .map_err(KalbaError::Io)
                    .and_then(|v| {
                        toml::from_str(&v).map_err(|e| KalbaError::TomlDe(String::from("data"), e))
                    })
            }) {
            Ok(v) => v,
            Err(e) => {
                if !matches!(e, KalbaError::Io(_)) {
                    can_save = false;
                    errors.push(e);
                }
                ToSave::default()
            }
        };

        let settings: Settings = match dirs::config_dir()
            .ok_or_else(|| KalbaError::MissingDir(String::from("config")))
            .and_then(|config_file| {
                fs::read_to_string(config_file.join("kalba.toml"))
                    .map_err(KalbaError::Io)
                    .and_then(|v| {
                        toml::from_str(&v)
                            .map_err(|e| KalbaError::TomlDe(String::from("config"), e))
                    })
            }) {
            Ok(v) => v,
            Err(e) => {
                if !matches!(e, KalbaError::Io(_)) {
                    can_save = false;
                    errors.push(e);
                }
                Settings::default()
            }
        };

        let language_cached_data = match dirs::cache_dir()
            .ok_or_else(|| KalbaError::MissingDir(String::from("cache")))
            .and_then(|cache_dir| {
                fs::read(cache_dir.join("kalba").join("dictionaries"))
                    .map_err(KalbaError::Io)
                    .and_then(|v| rmp_serde::from_slice(&v).map_err(KalbaError::CacheDecode))
            }) {
            Ok(v) => v,
            Err(e) => {
                if !matches!(e, KalbaError::Io(_)) {
                    errors.push(e);
                }
                HashMap::new()
            }
        };

        if let Err(e) = block_on(set_word_knowledge_from_anki(
            &mut to_save,
            &settings.languages,
            false,
            settings.anki_port,
        )) {
            errors.push(e);
        }

        if let Some(cmds) = &settings.to_run {
            for cmd in cmds {
                _ = run_command(cmd);
            }
        }

        let current_language = to_save
            .last_language
            .clone()
            .or_else(|| settings.languages.keys().next().cloned());
        for err in &errors {
            log::warn!("{}", err);
            log::warn!("{:?}", err);
        }

        Self {
            errors,
            to_save,
            settings,
            language_parser: None,
            current_language,
            dict_info: Default::default(),
            can_save,
            language_cached_data,
            in_reader: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct WordInfo {
    rating: i8,
    method: Method,
    history: Vec<(DateTime<Utc>, Method, i8)>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
enum Method {
    FromAnki,
    FromSeen,
    Specified,
    FromFrequency,
}

fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(KalbaState(Default::default()))
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
            start_stanza,
            refresh_anki,
            format_spyglys,
            get_spyglys_functions,
            time_spent,
            get_words_known_at_levels,
            get_startup_state,
            parse_url,
            read_file,
            get_definition_on_demand,
            always_change_lemma,
            setup_stanza,
            check_stanza_installed,
            get_export_variables,
            rename_language,
            get_words_added,
            get_started,
            get_url_contents,
            switch_page,
            check_version,
            uninstall_stanza
        ])
        .on_window_event(handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn refresh_anki(
    state: State<'_, KalbaState>,
    window: Window,
    force_all: bool,
) -> Result<(), KalbaError> {
    window
        .emit(
            "refresh_anki",
            ToasterPayload {
                message: Some("Loading anki data"),
            },
        )
        .unwrap();
    let mut state = state.0.lock().await;
    let languages = state.settings.languages.clone();
    let anki_port = state.settings.anki_port;

    set_word_knowledge_from_anki(&mut state.to_save, &languages, force_all, anki_port).await?;
    log::trace!("Anki data loaded [forced: {force_all}]");
    window.emit("refresh_anki", ToasterPayload { message: None })?;
    Ok(())
}

#[derive(Serialize, Debug)]
struct StartupState {
    errors: Vec<KalbaError>,
    first_time: bool,
    can_save: bool,
}

#[tauri::command]
async fn get_startup_state(state: State<'_, KalbaState>) -> Result<StartupState, String> {
    let mut state = state.0.lock().await;
    let errs = std::mem::take(&mut state.errors);
    Ok(StartupState {
        errors: errs,
        first_time: state.to_save.last_language.is_none(),
        can_save: state.can_save,
    })
}

#[tauri::command]
async fn get_started(
    state: State<'_, KalbaState>,
    starting: StartingSettings,
) -> Result<(), KalbaError> {
    let mut state = state.0.lock().await;
    let name = use_language_template(&mut state, &starting.template).await?;
    state
        .settings
        .languages
        .get_mut(&name)
        .expect("language to be added")
        .anki_parser = starting.decks;
    state.settings.stanza_enabled = starting.stanza_enabled;
    state.to_save.last_language = Some(name.clone());
    state.current_language = Some(name);
    save_state(&mut state)?;
    Ok(())
}

async fn set_word_knowledge_from_anki(
    to_save: &mut ToSave,
    languages: &HashMap<String, LanguageSettings>,
    force_all: bool,
    port: u16,
) -> Result<(), KalbaError> {
    let new_time = Utc::now();
    let days_passed = new_time
        .signed_duration_since(to_save.last_launched)
        .num_days()
        + 2;

    for (language_name, language) in languages {
        let to_save_language = to_save
            .language_specific
            .entry(language_name.to_owned())
            .or_default();
        for (deck, note_parser) in &language.anki_parser {
            get_anki_card_statuses(
                deck,
                &note_parser.0,
                &mut to_save_language.words,
                days_passed,
                // If the deck has not been added, it means this is the first time it is being
                // checked, so we should check every card and not just the ones recently
                // updated
                force_all || !to_save.decks_checked.contains(deck),
                port,
            )
            .await?;
            to_save.decks_checked.push(deck.to_owned());
        }

        if Some(&language.frequency_list) != to_save_language.previous_file.as_ref()
            || language.words_known_by_freq != to_save_language.previous_amount
        {
            to_save_language.previous_file = Some(language.frequency_list.clone());
            to_save_language.previous_amount = language.words_known_by_freq;
            update_words_known(
                &language.frequency_list,
                language.words_known_by_freq,
                &mut to_save_language.words,
            );
        }
    }
    to_save.last_launched = new_time;
    Ok(())
}

fn save_state(locked_state: &mut MutexGuard<SharedInfo>) -> Result<(), KalbaError> {
    if locked_state.can_save {
        log::info!("saving details");
        let saved_state_file = dirs::data_dir()
            .unwrap()
            .join("kalba")
            .join("saved_data.toml");
        locked_state.to_save.last_language = locked_state.current_language.clone();
        if locked_state.in_reader {
            locked_state.in_reader = false;
            let session = locked_state
                .to_save
                .sessions
                .last_mut()
                .expect("sessions should exist");
            session.1 = TimeDelta::to_std(&(Utc::now() - session.0)).expect("time should be valid");
            log::info!("saved session");
        }
        let conts = toml::to_string(&locked_state.to_save)?;
        fs::write(saved_state_file, conts)?;
    }
    Ok(())
}

fn handle_window_event(window: &Window, event: &WindowEvent) {
    block_on(async move {
        #[allow(clippy::single_match)] // Will probably be expanded in the future
        match event {
            WindowEvent::Destroyed => {
                let state: State<'_, KalbaState> = window.state();
                let mut locked_state = state.0.lock().await;
                let _ = save_state(&mut locked_state);
                let cache_file = dirs::cache_dir()
                    .expect("cache dir does not exist")
                    .join("kalba")
                    .join("dictionaries");
                fs::write(
                    cache_file,
                    rmp_serde::to_vec_named(&locked_state.language_cached_data)
                        .expect("error serializing cache"),
                )
                .expect("error writing cache");
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
    log::info!("updating words known");
    if let Ok(contents) = fs::read_to_string(file_name) {
        original_words.retain(|_, v| v.method != Method::FromFrequency);
        for word in contents.lines().take(words_known) {
            log::trace!("Checking word: {}", word);
            original_words.insert(
                word.to_owned(),
                WordInfo {
                    rating: 4,
                    method: Method::FromFrequency,
                    history: vec![(Utc::now(), Method::FromFrequency, 4)],
                },
            );
        }
    }
}

#[tauri::command]
async fn get_settings(state: State<'_, KalbaState>) -> Result<Settings, String> {
    log::trace!("Settings requested");
    let state = state.0.lock().await;
    Ok(state.settings.clone())
}

#[tauri::command]
async fn get_dark_mode(state: State<'_, KalbaState>) -> Result<bool, String> {
    let state = state.0.lock().await;
    Ok(state.settings.dark_mode)
}

#[tauri::command]
async fn get_rating(lemma: String, state: State<'_, KalbaState>) -> Result<i8, String> {
    log::trace!("Getting rating for word: {lemma}");
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
            history: vec![(Utc::now(), Method::FromSeen, 0)],
        })
        .rating)
}

#[tauri::command]
async fn get_languages(state: State<'_, KalbaState>) -> Result<Vec<String>, String> {
    let state = state.0.lock().await;
    Ok(state.settings.languages.keys().cloned().collect())
}

#[tauri::command]
async fn get_language(state: State<'_, KalbaState>) -> Result<Option<String>, String> {
    let state = state.0.lock().await;
    Ok(state.current_language.to_owned())
}

#[tauri::command]
async fn set_language(state: State<'_, KalbaState>, language: String) -> Result<(), String> {
    let mut state = state.0.lock().await;
    state.language_parser = None;
    state.current_language = Some(language);
    Ok(())
}

#[tauri::command]
async fn rename_language(
    state: State<'_, KalbaState>,
    original_name: String,
    new_name: String,
    new_selected_language: bool,
) -> Result<(), String> {
    let mut state = state.0.lock().await;
    log::info!("renaming template {original_name} to {new_name}");
    if let Some(old) = state.to_save.language_specific.remove(&original_name) {
        log::info!("moving saved state from {original_name} to {new_name}");
        state
            .to_save
            .language_specific
            .insert(new_name.clone(), old);
    }
    if let Some(old) = state.language_cached_data.remove(&original_name) {
        state.language_cached_data.insert(new_name.clone(), old);
    }
    if new_selected_language {
        state.current_language = Some(new_name);
    }
    Ok(())
}

#[tauri::command]
async fn write_settings(
    state: State<'_, KalbaState>,
    settings: Settings,
) -> Result<(), KalbaError> {
    let config_file = dirs::config_dir()
        .ok_or(KalbaError::MissingDir("config".to_string()))?
        .join("kalba.toml");
    let conts = toml::to_string_pretty(&settings)?;

    let mut state = state.0.lock().await;

    // TODO: avoid this clone (arc)
    let cloned_languages = state.settings.languages.clone();

    for (language, specific_settings) in &cloned_languages {
        if let Some(new_specific_settings) = settings.languages.get(language) {
            if new_specific_settings.dicts != specific_settings.dicts {
                if let Some(language_cache) = state.language_cached_data.get_mut(language) {
                    language_cache.definitions.clear();
                }
            }
        }
    }

    state.settings = settings;

    fs::write(config_file, conts)?;
    Ok(())
}

#[tauri::command]
async fn update_word_knowledge(
    state: State<'_, KalbaState>,
    word: &str,
    rating: i8,
    modifiable: bool,
) -> Result<(), String> {
    log::info!("Word {word} rating set to {rating}");
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

    let method = if modifiable {
        Method::FromAnki
    } else {
        Method::Specified
    };

    word_knowledge.history.push((Utc::now(), method, rating));
    word_knowledge.rating = rating;
    word_knowledge.method = method;
    Ok(())
}

#[tauri::command]
async fn always_change_lemma(
    state: State<'_, KalbaState>,
    lemma: String,
    updated_lemma: String,
) -> Result<(), String> {
    let mut state = state.0.lock().await;
    let language = state
        .current_language
        .clone()
        .expect("current language should already be chosen");
    state
        .to_save
        .language_specific
        .get_mut(&language)
        .expect("language should exist")
        .lemmas_to_replace
        .insert(lemma, updated_lemma);
    Ok(())
}

#[tauri::command]
async fn switch_page(state: State<'_, KalbaState>) -> Result<(), KalbaError> {
    let mut state = state.0.lock().await;
    if state.in_reader {
        save_state(&mut state)?;
    }
    Ok(())
}

#[tauri::command]
async fn check_version(
    state: State<'_, KalbaState>,
    potentially_new_version: String,
) -> Result<bool, String> {
    let mut state = state.0.lock().await;
    let is_new = state.to_save.kalba_version != potentially_new_version;
    if is_new {
        state.to_save.kalba_version = potentially_new_version;
    }
    Ok(is_new)
}
