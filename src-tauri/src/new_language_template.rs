use std::{collections::HashMap, fs};

use reqwest::Client;
use serde::Deserialize;
use shared::{Dictionary, LanguageSettings};
use tauri::State;
use tokio::sync::MutexGuard;

use crate::{KalbaError, KalbaState, SharedInfo};

#[derive(Debug, Deserialize, Clone)]
struct TemplateDetails {
    model: String,
    dicts: Vec<Dictionary>,
    frequency_list: bool,
    spyglys_details: bool,
    run_on_lemmas: Vec<String>,
    suggest_on_lemmas: Vec<String>,
    replace_lemmas: HashMap<String, String>,
}

#[tauri::command]
pub async fn new_language_from_template(
    state: State<'_, KalbaState>,
    language: &str,
) -> Result<String, KalbaError> {
    let mut state = state.0.lock().await;
    use_language_template(&mut state, language).await
}

pub async fn use_language_template(
    state: &mut MutexGuard<'_, SharedInfo>,
    language: &str,
) -> Result<String, KalbaError> {
    let language = language.to_lowercase();

    let mut language_name = language.clone();
    if state.settings.languages.contains_key(&language_name) {
        let mut language_number = 2;
        while state
            .settings
            .languages
            .contains_key(&format!("{language} {language_number}"))
        {
            language_number += 1;
        }
        language_name = format!("{language} {language_number}");
    }

    if language == "custom" {
        state
            .settings
            .languages
            .insert(language_name.clone(), LanguageSettings::default());
        state.to_save.language_specific.insert(
            language_name.clone(),
            crate::LanguageSpecificToSave::default(),
        );
        return Ok(language_name);
    }

    let client = Client::new();
    let template = client.get(format!(
        "https://raw.githubusercontent.com/brewingweasel/kalba/main/data/language_templates/{language}.toml",))
        .send()
        .await?
        .text()
        .await
        .expect("githubusercontent to return valid text");
    let details: TemplateDetails = toml::from_str(&template).unwrap();
    let frequency_list = if details.frequency_list {
        let data_dir = dirs::data_dir().ok_or_else(|| KalbaError::MissingDir("data".to_owned()))?;
        let path = data_dir
            .join("kalba")
            .join("language_data")
            .join(format!("{language}_frequency"));

        if data_dir.exists() {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
        }

        if !path.exists() {
            let contents = client.get(format!(
                "https://raw.githubusercontent.com/brewingweasel/kalba/main/data/frequency_lists/{language}",))
                .send()
                .await?
                .text()
                .await
                .expect("githubusercontent to return valid text");
            fs::write(&path, contents)?;
        }
        path.to_string_lossy().to_string()
    } else {
        String::new()
    };
    let grammar_parser = if details.spyglys_details {
        client.get(format!(
                "https://raw.githubusercontent.com/brewingweasel/kalba/main/data/spyglys/{language}.spyglys",))
                .send()
                .await?
                .text()
                .await
                .expect("githubusercontent to return valid text")
    } else {
        String::new()
    };

    let lang_settings = LanguageSettings {
        model: details.model,
        frequency_list,
        dicts: details.dicts,
        grammar_parser,
        run_on_lemmas: details.run_on_lemmas,
        suggest_on_lemmas: details.suggest_on_lemmas,
        ..Default::default()
    };
    state
        .settings
        .languages
        .insert(language_name.clone(), lang_settings);
    state.to_save.language_specific.insert(
        language_name.clone(),
        crate::LanguageSpecificToSave {
            lemmas_to_replace: details.replace_lemmas,
            ..Default::default()
        },
    );
    Ok(language_name)
}
