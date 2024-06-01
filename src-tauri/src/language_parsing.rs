use crate::SakinyjeState;
use shared::*;
use spacy_parsing::{get_spacy_info, PartOfSpeech};
use tauri::State;

#[tauri::command]
pub async fn parse_text(
    sent: &str,
    state: State<'_, SakinyjeState<'_>>,
) -> Result<Vec<Word>, String> {
    let mut state = state.0.lock().await;
    let mut words = Vec::new();
    if sent.is_empty() {
        return Ok(words);
    }
    let parsed_words = get_spacy_info(sent, &state.model)?;
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
    Ok(words)
}
