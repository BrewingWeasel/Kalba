use std::{collections::HashMap, fs};

use reqwest::Client;
use serde::Deserialize;
use shared::{Dictionary, LanguageSettings};
use tauri::State;

use crate::{SakinyjeError, SakinyjeState};

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
    state: State<'_, SakinyjeState>,
    language: String,
) -> Result<(), SakinyjeError> {
    let language = language.to_lowercase();
    if language == "custom" {
        return Ok(());
    }
    let client = Client::new();
    let template = client.get(format!(
        "https://raw.githubusercontent.com/brewingweasel/sakinyje/main/data/language_templates/{language}.toml",))
        .send()
        .await?
        .text()
        .await
        .expect("githubusercontent to return valid text");
    let details: TemplateDetails = toml::from_str(&template).unwrap();
    let frequency_list = if details.frequency_list {
        let path = dirs::data_dir()
            .ok_or_else(|| SakinyjeError::MissingDir("data".to_owned()))?
            .join("sakinyje")
            .join("language_data")
            .join(format!("{language}_frequency"));
        if !path.exists() {
            let contents = client.get(format!(
                "https://raw.githubusercontent.com/brewingweasel/sakinyje/main/data/frequency_lists/{language}",))
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
                "https://raw.githubusercontent.com/brewingweasel/sakinyje/main/data/spyglys/{language}.spyglys",))
                .send()
                .await?
                .text()
                .await
                .expect("githubusercontent to return valid text")
    } else {
        String::new()
    };
    let mut state = state.0.lock().await;
    let lang_settings = LanguageSettings {
        model: details.model,
        frequency_list,
        dicts: details.dicts,
        grammar_parser,
        run_on_lemmas: details.run_on_lemmas,
        suggest_on_lemmas: details.suggest_on_lemmas,
        ..Default::default()
    };
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
    state
        .settings
        .languages
        .insert(language_name.clone(), lang_settings);
    state.to_save.language_specific.insert(
        language_name,
        crate::LanguageSpecficToSave {
            lemmas_to_replace: details.replace_lemmas,
            ..Default::default()
        },
    );
    Ok(())
}
