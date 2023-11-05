use shared::*;
use spacy_parsing::{get_spacy_info, PartOfSpeech};

#[tauri::command]
pub async fn parse_text(sent: &str, model: &str) -> Result<Vec<Word>, String> {
    let mut words = Vec::new();
    let parsed_words = get_spacy_info(sent, model)?;
    for word in parsed_words {
        let clickable = !matches!(
            word.pos,
            PartOfSpeech::Punctuation | PartOfSpeech::Symbol | PartOfSpeech::Numeral
        );
        words.push(Word {
            text: word.text,
            clickable,
            lemma: word.lemma,
            morph: word.morph,
        });
    }
    Ok(words)
}
