use std::collections::HashMap;

use crate::SakinyjeState;
use shared::*;
use spacy_parsing::{get_spacy_info, PartOfSpeech};
use tauri::State;

#[tauri::command]
pub async fn parse_text(sent: &str, state: State<'_, SakinyjeState>) -> Result<Vec<Word>, String> {
    let mut state = state.0.lock().await;
    let language = state
        .current_language
        .clone()
        .expect("Language to have already been chosen");
    let mut words = Vec::new();
    if sent.is_empty() {
        return Ok(words);
    }
    if let Some(model) = &state.model {
        let parsed_words = get_spacy_info(sent, model)?;
        for word in parsed_words {
            let clickable = !matches!(
                word.pos,
                PartOfSpeech::Punctuation | PartOfSpeech::Symbol | PartOfSpeech::Numeral
            );
            let rating = state
                .to_save
                .language_specific
                .get_mut(&language)
                .expect("language to be chosen")
                .words
                .entry(word.lemma.clone())
                .or_insert(crate::WordInfo {
                    rating: 0,
                    method: crate::Method::FromSeen,
                })
                .rating;

            words.push(Word {
                text: word.text,
                clickable,
                rating,
                lemma: word.lemma,
                morph: word.morph,
            });
        }
    } else {
        let mut currently_building = String::new();
        let mut chars = sent.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_alphabetic() {
                currently_building.push(c);
            } else {
                if !currently_building.is_empty() {
                    let word = std::mem::take(&mut currently_building);
                    let rating = state
                        .to_save
                        .language_specific
                        .get_mut(&language)
                        .expect("language to be chosen")
                        .words
                        .entry(word.clone())
                        .or_insert(crate::WordInfo {
                            rating: 0,
                            method: crate::Method::FromSeen,
                        })
                        .rating;
                    words.push(Word {
                        text: word.clone(),
                        clickable: true,
                        lemma: word,
                        rating,
                        morph: HashMap::new(),
                    })
                }
                while let Some(possible_whitespace) = chars.peek() {
                    if possible_whitespace.is_whitespace() {
                        chars.next();
                    } else {
                        break;
                    }
                }
                if c.is_whitespace() {
                    continue;
                }
                words.push(Word {
                    text: c.to_string(),
                    clickable: false,
                    lemma: c.to_string(),
                    rating: 4,
                    morph: HashMap::new(),
                })
            }
        }
    }
    Ok(words)
}
