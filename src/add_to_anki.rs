use serde_json::json;
use std::error::Error;

use crate::settings::Settings;

pub fn add_to_anki(
    sent: &str,
    word: &str,
    defs: &Vec<String>,
    settings: &Settings,
) -> Result<(), Box<dyn Error>> {
    let mut def = String::new();
    for cur_def in defs {
        def.push_str(cur_def);
        def.push('\n');
    }
    let args = json!({
        "action": "addNote",
        "version": 6,
        "params": {
            "note": {
                "deckName": settings.deck,
                "modelName": settings.note_type,
                "fields": {
                    "Front": sent,
                    "Back": format!("{word}: {def}")
                },
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
    });
    let client = reqwest::blocking::Client::new();
    let _ = client.post("http://localhost:8765/").json(&args).send()?; // TODO: error handling
    Ok(())
}
