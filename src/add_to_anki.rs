use serde_json::json;
use std::error::Error;

pub fn add_to_anki(sent: &str, word: &str, def: &str) -> Result<(), Box<dyn Error>> {
    let args = json!({
        "action": "addNote",
        "version": 6,
        "params": {
            "note": {
                "deckName": "cool",
                "modelName": "Basic",
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
