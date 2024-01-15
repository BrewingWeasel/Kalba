use leptos::leptos_dom::logging::{console_error, console_log};
use serde::Serialize;
use shared::*;
use tauri_sys::tauri;

#[derive(Serialize)]
pub struct SettingsSaver {
    pub settings: Settings,
}

#[derive(Serialize)]
pub struct DeckRemover {
    pub deck: String,
}

#[derive(Clone, Serialize)]
struct GetFields {
    model: String,
}

pub fn save_settings(settings: Settings) {
    wasm_bindgen_futures::spawn_local(async move {
        console_log("saving settings");
        #[allow(clippy::single_match)]
        match tauri::invoke("write_settings", &SettingsSaver { settings }).await {
            Err(e) => console_error(&e.to_string()),
            Ok(()) => (),
        }
    })
}

pub fn remove_deck(deck: String) {
    wasm_bindgen_futures::spawn_local(async move {
        #[allow(clippy::single_match)]
        match tauri::invoke("remove_deck", &DeckRemover { deck }).await {
            Err(e) => console_error(&e.to_string()),
            Ok(()) => (),
        }
    })
}

pub async fn get_all_x_names(x: &str) -> Vec<String> {
    let note_or_deck = tauri::invoke::<(), Vec<String>>(&format!("get_all_{x}_names"), &()).await;
    match note_or_deck {
        Err(_) => Vec::new(),
        Ok(decks) => decks,
    }
}

pub async fn get_template_fields(template: String) -> Vec<String> {
    let note_or_deck = tauri::invoke("get_note_field_names", &GetFields { model: template }).await;
    match note_or_deck {
        Err(e) => {
            console_error(&format!("{}", e));
            Vec::new()
        }
        Ok(decks) => decks,
    }
}
