use reqwest::Client;
use serde_json::json;
use shared::Definition;
use std::{borrow::Cow, collections::HashMap};
use tauri::State;

use crate::{SakinyjeError, SakinyjeState};

fn get_json(export_details: ExportDetails<'_>) -> serde_json::Value {
    let mut def = String::new();
    for definition in export_details.defs.values() {
        def.push('\n');
        if let Definition::Text(t) = definition {
            def.push_str(t);
        }
    }

    let mut replacements = HashMap::from([
        (
            String::from("{sentence}"),
            Cow::Borrowed(export_details.sentence),
        ),
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
    state: State<'_, SakinyjeState>,
    language: String,
) -> Result<Vec<String>, SakinyjeError> {
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

#[derive(serde::Deserialize)]
pub struct ExportDetails<'a> {
    word: &'a str,
    sentence: &'a str,
    deck: &'a str,
    model: &'a str,
    fields: HashMap<&'a str, &'a str>,
    defs: HashMap<String, Definition>,
}

#[tauri::command]
pub async fn add_to_anki(export_details: ExportDetails<'_>) -> Result<(), SakinyjeError> {
    let args = get_json(export_details);
    let client = Client::new();
    client
        .post("http://localhost:8765/")
        .json(&args)
        .send()
        .await
        .map_err(|_| SakinyjeError::AnkiNotAvailable)?;
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
            sentence: "mmm",
            deck: "Default",
            model: "Basic",
            defs: HashMap::new(),
            fields: HashMap::from([("Front", "{sentence}"), ("Back", "{word}:")]),
        };
        let args = get_json(details);
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
            sentence: "sent:2",
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
        let args = get_json(details);
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
            &json!({"Front": "sent:2", "Back": "word:def1def2def3",})
        );
    }

    #[test]
    fn custom_fields_and_deck_defs_1() {
        let details = ExportDetails {
            word: "word",
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

        let args = get_json(details);
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
            sentence: "sent",
            deck: "deck",
            model: "note",
            defs: HashMap::new(),
            fields: HashMap::from([("sentence", "{sentence}"), ("sentence", "{sentence}")]),
        };
        let args = get_json(details);
        let params = args.get("params").unwrap();
        let note = params.get("note").unwrap();
        assert_eq!(note.get("fields").unwrap(), &json!({"sentence": "sent"}));
    }
}
