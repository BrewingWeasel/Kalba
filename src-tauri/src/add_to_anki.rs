use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;

use crate::SakinyjeError;

fn get_json(export_details: ExportDetails<'_>) -> serde_json::Value {
    let mut def = String::new();
    for cur_def in &export_details.defs {
        def.push('\n');
        def.push_str(cur_def);
    }

    let mut replacements = HashMap::from([
        (String::from("{sentence}"), export_details.sentence),
        (String::from("{word}"), export_details.word),
        (String::from("{def}"), &def),
    ]);

    for (i, v) in export_details.defs.iter().enumerate() {
        replacements.insert(format!("${}", i).to_owned(), v.as_str());
    }

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

#[derive(serde::Deserialize)]
pub struct ExportDetails<'a> {
    word: &'a str,
    sentence: &'a str,
    deck: &'a str,
    model: &'a str,
    fields: HashMap<&'a str, &'a str>,
    defs: Vec<String>,
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
    use shared::Settings;

    #[test]
    fn default_settings_no_defs() {
        let settings = Settings::default();
        let details = ExportDetails {
            word: "word",
            sentence: "mmm",
            deck: "Default",
            model: "Basic",
            defs: Vec::new(),
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
        let settings = Settings::default();

        let details = ExportDetails {
            word: "word",
            sentence: "sent:2",
            deck: "Default",
            model: "Basic",
            defs: vec![
                String::from("def1"),
                String::from("def2"),
                String::from("def3"),
            ],
            fields: HashMap::from([("Front", "{sentence}"), ("Back", "{word}:{def}")]),
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
            &json!({"Front": "sent:2", "Back": "word:
def1
def2
def3"})
        );
    }

    #[test]
    fn custom_fields_and_deck_defs_1() {
        let details = ExportDetails {
            word: "word",
            sentence: "sent",
            deck: "deck",
            model: "note",
            defs: vec![String::from("def1")],
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
            defs: Vec::new(),
            fields: HashMap::from([("sentence", "{sentence}"), ("sentence", "{sentence}")]),
        };
        let args = get_json(details);
        let params = args.get("params").unwrap();
        let note = params.get("note").unwrap();
        assert_eq!(note.get("fields").unwrap(), &json!({"sentence": "sent"}));
    }
}
