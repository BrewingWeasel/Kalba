// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{add_to_anki::add_to_anki, dictionary::get_defs, language_parsing::parse_text};
use ankiconnect::get_anki_card_statuses;
use chrono::{DateTime, Utc};
use commands::run_command;
use serde::{Deserialize, Serialize};
use shared::{SakinyjeResult, Settings};
use std::{collections::HashMap, error::Error, fs, thread, time::Duration};
use tauri::{
    async_runtime::{block_on, Mutex},
    AppHandle, GlobalWindowEvent, Manager, State, WindowEvent,
};

mod add_to_anki;
mod ankiconnect;
mod commands;
mod dictionary;
mod language_parsing;

struct SakinyjeState(tauri::async_runtime::Mutex<Result<SharedInfo, String>>);

#[derive(Default)]
struct SharedInfo {
    settings: Settings,
    to_save: ToSave,
}

#[derive(Serialize, Deserialize, Default)]
struct ToSave {
    words: HashMap<String, WordInfo>,
    cached_defs: HashMap<String, Vec<SakinyjeResult<String>>>,
    last_launched: DateTime<Utc>,
}

impl SharedInfo {
    fn get_shared_info() -> Result<Self, Box<dyn Error>> {
        let saved_state_file = dirs::data_dir()
            .ok_or("Unable to find directory for your platform")?
            .join("sakinyje_saved_data.toml");
        let config_file = dirs::config_dir()
            .ok_or("Unable to find directory for your platform")?
            .join("sakinyje.toml");

        let mut to_save: ToSave = fs::read_to_string(saved_state_file)
            .map(|v| toml::from_str(&v))?
            .unwrap_or_default();

        let settings: Settings = fs::read_to_string(config_file)
            .map(|v| toml::from_str(&v))?
            .unwrap_or_default();

        let new_time = Utc::now();
        let days_passed = new_time
            .signed_duration_since(to_save.last_launched)
            .num_days()
            + 1;

        if let Some(ankiparsers) = &settings.anki_parser {
            for (deck, note_parser) in ankiparsers {
                block_on(get_anki_card_statuses(
                    deck,
                    note_parser,
                    &mut to_save.words,
                    days_passed,
                ))?;
                // TODO: handle error
            }
        }

        if let Some(cmds) = &settings.to_run {
            for cmd in cmds {
                run_command(cmd)?;
            }
        }

        to_save.last_launched = new_time;
        Ok(Self { to_save, settings })
    }
}

#[derive(Serialize, Deserialize)]
struct WordInfo {
    rating: u8,
    can_modify: bool,
}

fn main() {
    tauri::Builder::default()
        .manage(SakinyjeState(Mutex::new(
            SharedInfo::get_shared_info().map_err(|e| e.to_string()),
        )))
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
        #[allow(clippy::single_match)] // Will probably be expanded in the future
        match event.event() {
            &WindowEvent::Destroyed => {
                let saved_state_file = dirs::data_dir().unwrap().join("sakinyje_saved_data.toml");
                let state: State<'_, SakinyjeState> = event.window().state();
                let lock = state.0.lock().await;
                let locked_state = lock.as_ref().unwrap();
                let conts =
                    toml::to_string(&locked_state.to_save).expect("Error serializing to toml");
                fs::write(saved_state_file, conts).expect("error writing to file");
            }
            _ => (),
        }
    })
}

#[tauri::command]
async fn get_settings(
    state: State<'_, SakinyjeState>,
    handle: AppHandle,
) -> Result<Settings, String> {
    let mut locked = state.0.lock().await;
    let state = ok_or_err_window(&mut *locked, handle)
        .await
        .expect("lol xd");
    Ok(state.settings.clone())
}

#[tauri::command]
async fn write_settings(
    state: State<'_, SakinyjeState>,
    handle: AppHandle,
    settings: Settings,
) -> Result<(), String> {
    let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");
    let conts = toml::to_string_pretty(&settings).unwrap();

    let mut locked = state.0.lock().await;
    let state = ok_or_err_window(&mut *locked, handle)
        .await
        .expect("lol xd");
    state.settings = settings;

    fs::write(config_file, conts).map_err(|e| e.to_string())
}

async fn ok_or_err_window(
    result: &mut Result<SharedInfo, String>,
    handle: AppHandle,
) -> Option<&mut SharedInfo> {
    match result {
        Ok(v) => Some(v),
        Err(e) => {
            let err = e.to_owned();
            std::thread::spawn(move || {
                if handle.get_window("error").is_none() {
                    let docs_window = tauri::WindowBuilder::new(
                        &handle,
                        "error",
                        tauri::WindowUrl::App(format!("error/{err}").into()),
                    )
                    .center()
                    .resizable(false)
                    .always_on_top(true)
                    .build()
                    .unwrap();
                    let _ = docs_window.show();
                }
            });
            // HACK: please help
            thread::sleep(Duration::from_secs(1000000));
            None
        }
    }
}
