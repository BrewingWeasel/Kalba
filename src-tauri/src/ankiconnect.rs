use std::collections::HashMap;

use reqwest::Response;
use serde::Deserialize;
use serde_json::{json, value::Value};

use crate::WordInfo;

pub struct NoteToWordHandling {
    pub field_to_use: String,
    pub remove_everything_in_parens: bool,
    pub only_first_word_or_line: bool,
}

#[derive(Deserialize, Debug)]
struct AnkiResult<T> {
    result: Option<T>,
    error: Option<String>,
}

impl<T> Into<Result<T, String>> for AnkiResult<T> {
    fn into(self) -> Result<T, String> {
        if let Some(r) = self.result {
            Ok(r)
        } else {
            Err(self.error.unwrap())
        }
    }
}

pub async fn get_anki_card_statuses(
    deck: &str,
    note_handling: HashMap<String, NoteToWordHandling>,
    original_words: &mut HashMap<String, WordInfo>,
) -> Result<(), String> {
    let find_cards_query = format!("deck:{deck}"); // TODO: only check cards reviewed since last
                                                   // check

    let cards = get_card_or_note_vals("findCards", json!({ "query": find_cards_query })).await?;
    let intervals = get_card_or_note_vals("getIntervals", json!({ "cards": &cards })).await?;
    let notes = get_card_or_note_vals("cardsToNotes", json!({ "cards": &cards })).await?;

    let words = get_words_from_notes(notes, note_handling).await?;
    for (word, interval) in words.into_iter().zip(intervals) {
        let rating = if interval <= 1 {
            0
        } else if interval <= 4 {
            1
        } else if interval <= 9 {
            2
        } else if interval <= 25 {
            4
        } else {
            5
        };

        if let Some(orig) = original_words.get_mut(&word) {
            if orig.can_modify {
                orig.rating = rating;
            }
        } else {
            original_words.insert(
                word,
                WordInfo {
                    rating,
                    can_modify: true,
                },
            );
        }
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct NoteInfo {
    fields: HashMap<String, FieldInfo>,
    #[serde(rename = "modelName")]
    model_name: String,
}

#[derive(Deserialize, Debug)]
struct FieldInfo {
    value: String,
}

async fn generic_anki_connect_action(action: &str, data: Value) -> Response {
    let client = reqwest::Client::new();
    let request = json!({
        "action": action,
        "version": 6,
        "params": data
    });

    client
        .post("http://127.0.0.1:8765")
        .json(&request)
        .send()
        .await
        .unwrap()
}

async fn get_card_or_note_vals(action: &str, data: Value) -> Result<Vec<usize>, String> {
    let res = generic_anki_connect_action(action, data).await;
    res.json::<AnkiResult<Vec<usize>>>().await.unwrap().into()
}

async fn get_words_from_notes(
    notes: Vec<usize>,
    templates: HashMap<String, NoteToWordHandling>,
) -> Result<Vec<String>, String> {
    let res = generic_anki_connect_action("notesInfo", json!({ "notes": notes })).await;
    let notes = Into::<Result<Vec<NoteInfo>, String>>::into(
        res.json::<AnkiResult<Vec<NoteInfo>>>().await.unwrap(),
    )?;

    let mut words = Vec::new();

    for note in notes {
        if let Some(handler) = templates.get(&note.model_name) {
            let selected_field = note
                .fields
                .into_iter()
                .find(|x| x.0 == handler.field_to_use)
                .unwrap()
                .1
                .value;
            words.push(get_word_from_field(selected_field, handler))
        } else {
            return Err(format!(
                "Unable to find model handler for {}",
                note.model_name
            ));
        }
    }
    Ok(words)
}

fn get_word_from_field(selected_field: String, handler: &NoteToWordHandling) -> String {
    let mut parsed = String::new();

    let mut in_bracket = false;

    for c in selected_field.chars() {
        match c {
            lp if lp == '[' || (lp == '(' && handler.remove_everything_in_parens) => {
                if !in_bracket {
                    in_bracket = true
                }
            }
            rp if rp == ']' || (rp == ')' && handler.remove_everything_in_parens) => {
                if in_bracket {
                    in_bracket = false
                }
            }
            ' ' | '\n' if handler.only_first_word_or_line => break,
            c if !in_bracket => parsed.push(c),
            _ => (),
        }
    }
    parsed
}
