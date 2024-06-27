use std::fs;

use reqwest::Client;
use serde::Deserialize;
use shared::{Dictionary, LanguageSettings};
use tauri::State;

use crate::SakinyjeState;

#[derive(Debug, Deserialize, Clone)]
struct TemplateDetails {
    model: String,
    dicts: Vec<(String, Dictionary)>,
    frequency_list: bool,
}

#[tauri::command]
pub async fn new_language_from_template(
    state: State<'_, SakinyjeState>,
    language: String,
) -> Result<(), String> {
    let language = language.to_lowercase();
    if language == "custom" {
        return Ok(Default::default());
    }
    let client = Client::new();
    let template = client.get(format!(
        "https://raw.githubusercontent.com/brewingweasel/sakinyje/main/data/language_templates/{language}",))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let details: TemplateDetails = toml::from_str(&template).unwrap();
    let frequency_list = if details.frequency_list {
        let path = dirs::data_dir()
            .unwrap()
            .join("sakinyje")
            .join("language_data")
            .join(format!("{language}_frequency"));
        if !path.exists() {
            let contents = client.get(format!(
                "https://raw.githubusercontent.com/brewingweasel/sakinyje/main/data/frequency_lists/{language}",))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            fs::write(&path, contents).unwrap();
        }
        path.to_string_lossy().to_string()
    } else {
        String::new()
    };
    let mut state = state.0.lock().await;
    let lang_settings = LanguageSettings {
        model: details.model,
        frequency_list,
        dicts: details.dicts,
        ..Default::default()
    };
    if state.settings.languages.contains_key(&language) {
        let mut language_number = 2;
        while state
            .settings
            .languages
            .contains_key(&format!("{language} {language_number}"))
        {
            language_number += 1;
        }
        state
            .settings
            .languages
            .insert(format!("{language} {language_number}"), lang_settings);
    } else {
        state.settings.languages.insert(language, lang_settings);
    }
    Ok(())
}
