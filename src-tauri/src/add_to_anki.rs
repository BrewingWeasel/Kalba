use chrono::Utc;
use reqwest::Client;
use serde_json::json;
use shared::{Definition, ExportStyling, ToasterPayload};
use std::{borrow::Cow, collections::HashMap};
use tauri::{Emitter, State, Window};

use crate::{ankiconnect::AnkiResult, KalbaError, KalbaState};

fn get_json(
    export_details: ExportDetails<'_>,
    export_styling: &ExportStyling,
) -> serde_json::Value {
    let mut def = String::new();
    for definition in export_details.defs.values() {
        def.push('\n');
        if let Definition::Text(t) = definition {
            def.push_str(t);
        }
    }

    let updated_sentence = export_details.sentence.replace(
        export_details.original_form,
        &format!(
            "<span style=\"{}\">{}</span>",
            export_styling.word_in_sentence, export_details.original_form
        ),
    );

    let mut replacements = HashMap::from([
        (String::from("{sentence}"), Cow::Owned(updated_sentence)),
        (String::from("{word}"), Cow::Borrowed(export_details.word)),
        (String::from("{def}"), Cow::Owned(def)),
    ]);

    for (name, value) in export_details.defs {
        replacements.insert(
            format!("{{def:{}}}", name).to_owned(),
            match value {
                Definition::Text(t) => Cow::Owned(t),
                Definition::Empty => Cow::Borrowed(""),
                Definition::OnDemand(_) => Cow::Borrowed(""),
            },
        );
    }
    log::debug!("Possible replacements: {:?}", replacements);

    let mut fields = HashMap::new();

    for (field_name, conts) in &export_details.fields {
        let mut conts = conts.to_string();
        for (orig, replacement) in &replacements {
            conts = conts.replace(orig, replacement);
        }
        fields.insert(field_name, conts);
    }

    json!({
        "action": "addNote",
        "version": 6,
        "params": {
            "note": {
                "deckName": export_details.deck,
                "modelName": export_details.model,
                "fields": fields,
                "options": {
                    "allowDuplicate": false,
                    "duplicateScope": "deck",
                    "duplicateScopeOptions": {
                        "deckName": export_details.deck,
                        "checkChildren": false,
                        "checkAllModels": false
                    }
                },
            }
        }
    })
}

#[tauri::command]
pub async fn get_export_variables(
    state: State<'_, KalbaState>,
    language: String,
) -> Result<Vec<String>, KalbaError> {
    let state = state.0.lock().await;

    let mut export_variables = Vec::new();

    for dict in state
        .settings
        .languages
        .get(&language)
        .expect("Language exists")
        .dicts
        .iter()
    {
        export_variables.push(dict.name.clone());
    }
    Ok(export_variables)
}

#[derive(serde::Deserialize, Debug)]
pub struct ExportDetails<'a> {
    word: &'a str,
    original_form: &'a str,
    sentence: &'a str,
    deck: &'a str,
    model: &'a str,
    fields: HashMap<&'a str, &'a str>,
    defs: HashMap<String, Definition>,
}

#[tauri::command]
pub async fn add_to_anki(
    export_details: ExportDetails<'_>,
    state: State<'_, KalbaState>,
    window: Window,
) -> Result<(), KalbaError> {
    let mut state = state.0.lock().await;
    log::debug!("Adding to anki using details {:?}", export_details);
    let selected_word = export_details.word;
    let args = get_json(export_details, &state.settings.export_styling);
    let client = Client::new();
    let url = format!("http://localhost:{}/", state.settings.anki_port);
    let response = client
        .post(url)
        .json(&args)
        .send()
        .await
        .map_err(|_| KalbaError::AnkiNotAvailable)?;
    std::convert::Into::<Result<isize, KalbaError>>::into(
        response.json::<AnkiResult<isize>>().await?,
    )?;
    let current_language = state.current_language.clone().expect("language to exist");

    state
        .to_save
        .language_specific
        .get_mut(&current_language)
        .expect("language to exist")
        .added_to_anki
        .push((Utc::now(), selected_word.to_string()));
    window.emit(
        "added_to_anki",
        Some(ToasterPayload {
            message: Some(&format!("Added {} to anki", selected_word)),
        }),
    )?;
    log::debug!("Added to anki");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Value;

    #[test]
    fn default_settings_no_defs() {
        let details = ExportDetails {
            word: "word",
            original_form: "word",
            sentence: "mmm",
            deck: "Default",
            model: "Basic",
            defs: HashMap::new(),
            fields: HashMap::from([("Front", "{sentence}"), ("Back", "{word}:")]),
        };
        let args = get_json(details, &ExportStyling::default());
        let params = args.get("params").unwrap();
        let note = params.get("note").unwrap();
        assert_eq!(
            note.get("deckName").unwrap(),
            &Value::String(String::from("Default"))
        );
        assert_eq!(
            note.get("modelName").unwrap(),
            &Value::String(String::from("Basic"))
        );
        assert_eq!(
            note.get("fields").unwrap(),
            &json!({"Front": "mmm", "Back": "word:"})
        );
    }

    #[test]
    fn default_settings_defs() {
        let details = ExportDetails {
            word: "word",
            original_form: "word",
            sentence: "sent with word:2",
            deck: "Default",
            model: "Basic",
            defs: HashMap::from([
                (
                    String::from("dict1"),
                    Definition::Text(String::from("def1")),
                ),
                (
                    String::from("dict2"),
                    Definition::Text(String::from("def2")),
                ),
                (
                    String::from("dict3"),
                    Definition::Text(String::from("def3")),
                ),
            ]),
            fields: HashMap::from([
                ("Front", "{sentence}"),
                ("Back", "{word}:{def:dict1}{def:dict2}{def:dict3}"),
            ]),
        };
        let args = get_json(details, &ExportStyling::default());
        let params = args.get("params").unwrap();
        let note = params.get("note").unwrap();
        assert_eq!(
            note.get("deckName").unwrap(),
            &Value::String(String::from("Default"))
        );
        assert_eq!(
            note.get("modelName").unwrap(),
            &Value::String(String::from("Basic"))
        );
        assert_eq!(
            note.get("fields").unwrap(),
            &json!({"Front": "sent with <span style=\"color: #ea9a97; font-weight: 800; font-style: italic;\">word</span>:2", "Back": "word:def1def2def3",})
        );
    }

    #[test]
    fn custom_fields_and_deck_defs_1() {
        let details = ExportDetails {
            word: "word",
            original_form: "word",
            sentence: "sent",
            deck: "deck",
            model: "note",
            defs: HashMap::from([(
                String::from("dict1"),
                Definition::Text(String::from("def1")),
            )]),
            fields: HashMap::from([
                ("sentence", "{sentence}[{word}]"),
                ("word", "{word}"),
                ("def", "{def}"),
            ]),
        };

        let args = get_json(details, &ExportStyling::default());
        let params = args.get("params").unwrap();
        let note = params.get("note").unwrap();
        assert_eq!(
            note.get("deckName").unwrap(),
            &Value::String(String::from("deck"))
        );
        assert_eq!(
            note.get("modelName").unwrap(),
            &Value::String(String::from("note"))
        );
        assert_eq!(
            note.get("fields").unwrap(),
            &json!({"sentence": "sent[word]", "word": "word", "def": "\ndef1"})
        );
    }

    #[test]
    fn custom_fields_and_deck_same_field_twice() {
        let details = ExportDetails {
            word: "word",
            original_form: "word",
            sentence: "sent",
            deck: "deck",
            model: "note",
            defs: HashMap::new(),
            fields: HashMap::from([("sentence", "{sentence}"), ("sentence", "{sentence}")]),
        };
        let args = get_json(details, &ExportStyling::default());
        let params = args.get("params").unwrap();
        let note = params.get("note").unwrap();
        assert_eq!(note.get("fields").unwrap(), &json!({"sentence": "sent"}));
    }
}
