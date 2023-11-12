use reqwest::Client;
use serde_json::json;
use shared::Settings;
use std::collections::HashMap;
use tauri::{AppHandle, State};

use crate::{ok_or_err_window, SakinyjeState};

fn get_json(
    sent: &str,
    word: &str,
    defs: Vec<String>,
    settings: &Settings,
) -> Result<serde_json::Value, String> {
    let mut def = String::new();
    for cur_def in &defs {
        def.push('\n');
        def.push_str(cur_def);
    }

    let mut replacements = HashMap::from([
        (String::from("{sent}"), sent),
        (String::from("{word}"), word),
        (String::from("{def}"), &def),
    ]);

    for (i, v) in defs.iter().enumerate() {
        replacements.insert(format!("${}", i).to_owned(), v);
    }

    let mut fields = HashMap::new();

    for i in settings.note_fields.lines() {
        let (field_name, conts) = i.split_once(':').ok_or("error parsing fields")?;
        let mut conts = conts.to_string();
        for (orig, replacement) in &replacements {
            conts = conts.replace(orig, replacement);
        }
        fields.insert(field_name, conts);
    }

    Ok(json!({
        "action": "addNote",
        "version": 6,
        "params": {
            "note": {
                "deckName": settings.deck,
                "modelName": settings.note_type,
                "fields": fields,
                "options": {
                    "allowDuplicate": false,
                    "duplicateScope": "deck",
                    "duplicateScopeOptions": {
                        "deckName": settings.deck,
                        "checkChildren": false,
                        "checkAllModels": false
                    }
                },
            }
        }
    }))
}

#[tauri::command]
pub async fn add_to_anki(
    sent: &str,
    word: &str,
    defs: Vec<String>,
    state: State<'_, SakinyjeState>,
    handle: AppHandle,
) -> Result<(), String> {
    let mut locked = state.0.lock().await;
    let state = ok_or_err_window(&mut *locked, handle)
        .await
        .expect("lol xd");
    let args = get_json(sent, word, defs, &state.settings).map_err(|e| e.to_string())?;
    let client = Client::new();
    client
        .post("http://localhost:8765/")
        .json(&args)
        .send()
        .await
        .map_err(|e| e.to_string())?;
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
        let args = get_json("mmm", "word", Vec::new(), &settings).unwrap();
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
        let args = get_json(
            "sent:2",
            "word",
            vec![
                String::from("def1"),
                String::from("def2"),
                String::from("def3"),
            ],
            &settings,
        )
        .unwrap();
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
        let settings = Settings {
            deck: String::from("deck"),
            note_type: String::from("note"),
            note_fields: String::from(
                "sentence:{sent}[{word}]
word:{word}
def:{def}",
            ),
            ..Default::default()
        };
        let args = get_json("sent", "word", vec![String::from("def1")], &settings).unwrap();
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
        let settings = Settings {
            deck: String::from("deck"),
            note_type: String::from("note"),
            note_fields: String::from(
                "sentence:{sent}
sentence:{word}",
            ),
            ..Default::default()
        };
        let args = get_json("sent", "word", Vec::new(), &settings).unwrap();
        let params = args.get("params").unwrap();
        let note = params.get("note").unwrap();
        assert_eq!(note.get("fields").unwrap(), &json!({"sentence": "word"}));
    }
}
