use crate::{ok_or_err_window, SakinyjeState};
use shared::*;
use spacy_parsing::{get_spacy_info, PartOfSpeech};
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn parse_text(
    sent: &str,
    model: &str,
    state: State<'_, SakinyjeState>,
    handle: AppHandle,
) -> Result<Vec<Word>, String> {
    let mut locked = state.0.lock().await;
    let state = ok_or_err_window(&mut *locked, handle)
        .await
        .expect("lol xd");
    let mut words = Vec::new();
    if sent.is_empty() {
        return Ok(words);
    }
    let parsed_words = get_spacy_info(sent, model)?;
    for word in parsed_words {
        let clickable = !matches!(
            word.pos,
            PartOfSpeech::Punctuation | PartOfSpeech::Symbol | PartOfSpeech::Numeral
        );
        let rating = state
            .to_save
            .words
            .entry(word.lemma.clone())
            .or_insert(crate::WordInfo {
                rating: 0,
                can_modify: true,
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
    Ok(words)
}
