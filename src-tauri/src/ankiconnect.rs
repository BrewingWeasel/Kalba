use std::collections::HashMap;

use chrono::Utc;
use reqwest::Response;
use serde::Deserialize;
use serde_json::{json, value::Value};
use shared::NoteToWordHandling;
use tauri::State;

use crate::{Method, SakinyjeError, SakinyjeState, WordInfo};

#[derive(Deserialize, Debug)]
struct AnkiResult<T> {
    result: Option<T>,
    error: Option<String>,
}

impl<T> From<AnkiResult<T>> for Result<T, SakinyjeError> {
    fn from(val: AnkiResult<T>) -> Self {
        if let Some(r) = val.result {
            Ok(r)
        } else {
            Err(SakinyjeError::AnkiConnectError(
                val.error.expect("either an error or a value"),
            ))
        }
    }
}

pub async fn get_anki_card_statuses(
    deck: &str,
    note_handling: &HashMap<String, NoteToWordHandling>,
    original_words: &mut HashMap<String, WordInfo>,
    days_passed: i64,
    first_time_run: bool,
) -> Result<(), SakinyjeError> {
    log::info!("getting anki card statuses");
    let days_passed_query = if first_time_run {
        String::new()
    } else {
        format!("rated:{days_passed}")
    };
    let find_cards_query = format!("\"deck:{deck}\"{days_passed_query}");
    log::info!("Using query: {find_cards_query}");

    let cards = get_card_or_note_vals("findCards", json!({ "query": find_cards_query })).await?;
    let intervals = get_card_or_note_vals("getIntervals", json!({ "cards": &cards })).await?;
    let notes = get_card_or_note_vals("cardsToNotes", json!({ "cards": &cards })).await?;

    let res = generic_anki_connect_action("notesInfo", json!({ "notes": notes })).await?;
    let notes_info = Into::<Result<Vec<NoteInfo>, SakinyjeError>>::into(
        res.json::<AnkiResult<Vec<NoteInfo>>>()
            .await
            .expect("valid json from anki"),
    )?;

    for ((note, interval), note_info) in notes.into_iter().zip(intervals).zip(notes_info) {
        let Ok(Some(word)) = get_word_from_note(note, note_info, note_handling).await else {
            continue;
        };

        log::trace!("found word {word} with interval {interval}");

        let rating = if interval <= 1 {
            1
        } else if interval <= 9 {
            2
        } else if interval <= 23 {
            3
        } else {
            4
        };

        if let Some(orig) = original_words.get_mut(&word) {
            if orig.method != Method::Specified {
                orig.rating = rating;
            }
        } else {
            original_words.insert(
                word,
                WordInfo {
                    rating,
                    method: Method::FromAnki,
                    history: vec![(Utc::now(), Method::FromAnki, rating)],
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

async fn generic_anki_connect_action(action: &str, data: Value) -> Result<Response, SakinyjeError> {
    let client = reqwest::Client::new();
    let request = if data == Value::Null {
        json!({
            "action": action,
            "version": 6,
        })
    } else {
        json!({
            "action": action,
            "version": 6,
            "params": data
        })
    };

    client
        .post("http://127.0.0.1:8765")
        .json(&request)
        .send()
        .await
        .map_err(|_| SakinyjeError::AnkiNotAvailable)
}

async fn get_card_or_note_vals(action: &str, data: Value) -> Result<Vec<isize>, SakinyjeError> {
    let res = generic_anki_connect_action(action, data).await?;
    res.json::<AnkiResult<Vec<isize>>>().await.unwrap().into()
}

async fn get_word_from_note(
    note_id: isize,
    note: NoteInfo,
    templates: &HashMap<String, NoteToWordHandling>,
) -> Result<Option<String>, SakinyjeError> {
    if let Some(handler) = templates.get(&note.model_name) {
        let res = generic_anki_connect_action("getNoteTags", json!({ "note": &note_id })).await?;
        let tags: Result<Vec<String>, SakinyjeError> = res
            .json::<AnkiResult<Vec<String>>>()
            .await
            .expect("Valid json from anki")
            .into();

        if !handler.tags_wanted.is_empty() && !tags?.iter().any(|t| handler.tags_wanted.contains(t))
        {
            return Ok(None);
        }

        let selected_field = note
            .fields
            .into_iter()
            .find(|x| x.0 == handler.field_to_use)
            .unwrap()
            .1
            .value;
        Ok(Some(get_word_from_field(selected_field, handler)))
    } else {
        Err(SakinyjeError::NoModelHandler)
    }
}

fn get_word_from_field(selected_field: String, handler: &NoteToWordHandling) -> String {
    let mut parsed = String::new();

    let mut in_bracket = false;

    for c in selected_field.replace("&nbsp;", " ").chars() {
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
    parsed.trim().to_owned()
}

#[tauri::command]
pub async fn get_all_deck_names() -> Result<Vec<String>, SakinyjeError> {
    let res = generic_anki_connect_action("deckNames", Value::Null).await?;
    res.json::<AnkiResult<Vec<String>>>()
        .await
        .expect("Valid json from anki")
        .into()
}

#[tauri::command]
pub async fn get_all_note_names() -> Result<Vec<String>, SakinyjeError> {
    let res = generic_anki_connect_action("modelNames", Value::Null).await?;
    res.json::<AnkiResult<Vec<String>>>()
        .await
        .expect("Valid json from anki")
        .into()
}

#[tauri::command]
pub async fn get_note_field_names(model: &str) -> Result<Vec<String>, SakinyjeError> {
    let res = generic_anki_connect_action("modelFieldNames", json!({ "modelName": model })).await?;
    res.json::<AnkiResult<Vec<String>>>()
        .await
        .expect("Valid json from anki")
        .into()
}

#[tauri::command]
pub async fn remove_deck(deck: String, state: State<'_, SakinyjeState>) -> Result<(), String> {
    state
        .0
        .lock()
        .await
        .to_save
        .decks_checked
        .retain(|v| v != &deck);
    Ok(())
}
