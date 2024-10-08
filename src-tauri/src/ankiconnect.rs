use std::{collections::HashMap, time::Duration};

use chrono::Utc;
use reqwest::Response;
use serde::Deserialize;
use serde_json::{json, value::Value};
use shared::NoteToWordHandling;
use tauri::State;

use crate::{KalbaError, KalbaState, Method, WordInfo};

#[derive(Deserialize, Debug)]
pub struct AnkiResult<T> {
    result: Option<T>,
    error: Option<String>,
}

impl<T> From<AnkiResult<T>> for Result<T, KalbaError> {
    fn from(val: AnkiResult<T>) -> Self {
        if let Some(r) = val.result {
            Ok(r)
        } else {
            Err(KalbaError::AnkiConnectError(
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
    port: u16,
) -> Result<(), KalbaError> {
    log::info!("getting anki card statuses");
    let client = reqwest::Client::new();
    let days_passed_query = if first_time_run {
        String::new()
    } else {
        format!("rated:{days_passed}")
    };
    for (note_type, handler) in note_handling {
        let find_notes_query = format!(
            "\"deck:{deck}\" \"note:{note_type}\" {days_passed_query} {}",
            handler.search_params
        );
        log::info!("Using query: {find_notes_query}");
        let notes = get_card_or_note_vals(
            "findNotes",
            json!({ "query": find_notes_query }),
            &client,
            port,
        )
        .await?;

        let notes_info_res =
            generic_anki_connect_action("notesInfo", json!({ "notes": notes }), &client, port)
                .await?;
        let notes_info = Into::<Result<Vec<NoteInfo>, KalbaError>>::into(
            notes_info_res
                .json::<AnkiResult<Vec<NoteInfo>>>()
                .await
                .expect("valid json from anki"),
        )?;
        for note in &notes_info {
            let word = get_word_from_note(note, handler).await;

            let intervals = get_card_or_note_vals(
                "getIntervals",
                json!({ "cards": note.cards }),
                &client,
                port,
            )
            .await?;
            let selected_interval = intervals.iter().max().copied().unwrap_or_default();
            log::trace!("found word {word} with interval {selected_interval}");

            // TODO: these intervals should be configurable
            let rating = if selected_interval <= 1 {
                1
            } else if selected_interval <= 9 {
                2
            } else if selected_interval <= 23 {
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
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct NoteInfo {
    fields: HashMap<String, FieldInfo>,
    cards: Vec<isize>,
}

#[derive(Deserialize, Debug)]
struct FieldInfo {
    value: String,
}

async fn generic_anki_connect_action(
    action: &str,
    data: Value,
    client: &reqwest::Client,
    port: u16,
) -> Result<Response, KalbaError> {
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

    let url = format!("http://127.0.0.1:{port}");

    match client.post(&url).json(&request).send().await {
        Ok(r) => Ok(r),
        Err(_) => {
            // try again after a short delay
            tokio::time::sleep(Duration::from_millis(300)).await;
            client
                .post(url)
                .json(&request)
                .send()
                .await
                .map_err(|_| KalbaError::AnkiNotAvailable)
        }
    }
}

async fn get_card_or_note_vals(
    action: &str,
    data: Value,
    client: &reqwest::Client,
    port: u16,
) -> Result<Vec<isize>, KalbaError> {
    let res = generic_anki_connect_action(action, data, client, port).await?;
    res.json::<AnkiResult<Vec<isize>>>().await.unwrap().into()
}

async fn get_word_from_note(note: &NoteInfo, handler: &NoteToWordHandling) -> String {
    let selected_field = &note
        .fields
        .iter()
        .find(|x| x.0 == &handler.field_to_use)
        .unwrap()
        .1
        .value;
    get_word_from_field(selected_field, handler)
}

fn get_word_from_field(selected_field: &str, handler: &NoteToWordHandling) -> String {
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
pub async fn get_all_deck_names(state: State<'_, KalbaState>) -> Result<Vec<String>, KalbaError> {
    let port = state.0.lock().await.settings.anki_port;
    let res = generic_anki_connect_action("deckNames", Value::Null, &reqwest::Client::new(), port)
        .await?;
    res.json::<AnkiResult<Vec<String>>>()
        .await
        .expect("Valid json from anki")
        .into()
}

#[tauri::command]
pub async fn get_all_note_names(state: State<'_, KalbaState>) -> Result<Vec<String>, KalbaError> {
    let port = state.0.lock().await.settings.anki_port;
    let res = generic_anki_connect_action("modelNames", Value::Null, &reqwest::Client::new(), port)
        .await?;
    res.json::<AnkiResult<Vec<String>>>()
        .await
        .expect("Valid json from anki")
        .into()
}

#[tauri::command]
pub async fn get_note_field_names(
    state: State<'_, KalbaState>,
    model: &str,
) -> Result<Vec<String>, KalbaError> {
    let port = state.0.lock().await.settings.anki_port;
    let res = generic_anki_connect_action(
        "modelFieldNames",
        json!({ "modelName": model }),
        &reqwest::Client::new(),
        port,
    )
    .await?;
    res.json::<AnkiResult<Vec<String>>>()
        .await
        .expect("Valid json from anki")
        .into()
}

#[tauri::command]
pub async fn remove_deck(deck: String, state: State<'_, KalbaState>) -> Result<(), String> {
    state
        .0
        .lock()
        .await
        .to_save
        .decks_checked
        .retain(|v| v != &deck);
    Ok(())
}
