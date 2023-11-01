use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;

use shared::Settings;

fn get_json(
    sent: &str,
    word: &str,
    defs: Vec<String>,
    settings: Settings,
) -> Result<serde_json::Value, String> {
    let mut def = String::new();
    for cur_def in &defs {
        def.push_str(cur_def);
        def.push('\n');
    }

    let mut replacements = HashMap::from([
        (String::from("$sent"), sent),
        (String::from("$word"), word),
        (String::from("$def"), &def),
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
                        "deckName": "Default",
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
    settings: Settings,
) -> Result<(), String> {
    let args = get_json(sent, word, defs, settings).map_err(|e| e.to_string())?;
    let client = Client::new();
    client
        .post("http://localhost:8765/")
        .json(&args)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
