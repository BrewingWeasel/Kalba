use serde_json::json;
use std::{collections::HashMap, error::Error};

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

    let args = json!({
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
    });
    let client = reqwest::blocking::Client::new();
    let _ = client.post("http://localhost:8765/").json(&args).send()?; // TODO: error handling
    Ok(())
}
