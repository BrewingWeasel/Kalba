use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use serde_map_to_array::{HashMapToArray, KeyValueLabels};

struct DeckKeyValueLabels;

impl KeyValueLabels for DeckKeyValueLabels {
    const KEY: &'static str = "name";
    const VALUE: &'static str = "notes";
}

struct NoteKeyValueLabels;

impl KeyValueLabels for NoteKeyValueLabels {
    const KEY: &'static str = "model";
    const VALUE: &'static str = "handling";
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Word {
    pub text: String,
    pub lemma: String,
    pub rating: u8,
    pub morph: HashMap<String, String>,
    pub clickable: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Debug)]
#[serde(tag = "t", content = "c")]
pub enum DictFileType {
    TextSplitAt(String),
    StarDict,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Dictionary {
    File(String, DictFileType),
    Url(String),
    Command(String),
    EkalbaBendrines,
    EkalbaDabartines,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "t", content = "conts")]
pub enum SakinyjeResult<T> {
    Ok(T),
    Err(String),
}

impl<T, E> From<Result<T, E>> for SakinyjeResult<T>
where
    E: ToString,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(e.to_string()),
        }
    }
}

impl<T> From<SakinyjeResult<T>> for Result<T, String> {
    fn from(value: SakinyjeResult<T>) -> Self {
        match value {
            SakinyjeResult::Ok(v) => Self::Ok(v),
            SakinyjeResult::Err(e) => Self::Err(e),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NoteToWordHandling {
    pub field_to_use: String,
    pub remove_everything_in_parens: bool,
    pub only_first_word_or_line: bool,
    pub tags_wanted: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]

pub struct Note(
    #[serde(with = "HashMapToArray::<String, NoteToWordHandling, NoteKeyValueLabels>")]
    pub  HashMap<String, NoteToWordHandling>,
);

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    pub deck: String,
    pub note_type: String,
    pub note_fields: HashMap<String, String>,
    pub model: String,
    pub dicts: Vec<Dictionary>,
    pub to_remove: Option<usize>,
    pub css: Option<String>,
    #[serde(with = "HashMapToArray::<String, Note, DeckKeyValueLabels>")]
    pub anki_parser: HashMap<String, Note>,
    pub to_run: Option<Vec<String>>,
    pub dark_mode: bool,
    pub frequency_list: String,
    pub words_known_by_freq: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            deck: String::from(""),
            note_type: String::from(""),
            note_fields: HashMap::from([
                (String::from("Front"), String::from("{sent}")),
                (String::from("Back"), String::from("{word}:{def}")),
            ]),
            model: String::from("lt_core_news_sm"),
            dicts: Vec::new(),
            to_remove: None,
            css: None,
            anki_parser: HashMap::new(),
            to_run: None,
            dark_mode: true,
            frequency_list: String::from(""),
            words_known_by_freq: 0,
        }
    }
}
