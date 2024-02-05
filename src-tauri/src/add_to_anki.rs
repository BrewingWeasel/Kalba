use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;

fn get_json(export_details: ExportDetails<'_>) -> Result<serde_json::Value, String> {
    let mut def = String::new();
    for cur_def in &export_details.defs {
        def.push('\n');
        def.push_str(cur_def);
    }

    let mut replacements = HashMap::from([
        (String::from("{sent}"), export_details.sentence),
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

    Ok(json!({
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
    }))
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
pub async fn add_to_anki(export_details: ExportDetails<'_>) -> Result<(), String> {
    let args = get_json(export_details).map_err(|e| e.to_string())?;
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
            note_fields: HashMap::from([
                (String::from("sentence"), String::from("{sent}[{word}]")),
                (String::from("word"), String::from("{word}")),
                (String::from("def"), String::from("{def}")),
            ]),
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
            note_fields: HashMap::from([
                (String::from("sentence"), String::from("{sent}")),
                (String::from("sentence"), String::from("{word}")),
            ]),
            ..Default::default()
        };
        let args = get_json("sent", "word", Vec::new(), &settings).unwrap();
        let params = args.get("params").unwrap();
        let note = params.get("note").unwrap();
        assert_eq!(note.get("fields").unwrap(), &json!({"sentence": "word"}));
    }
}
