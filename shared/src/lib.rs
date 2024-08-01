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

#[derive(Clone, serde::Serialize)]
pub struct ToasterPayload<'a> {
    pub message: Option<&'a str>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Word {
    pub text: String,
    pub lemma: String,
    pub rating: i8,
    pub sentence_index: usize,
    pub morph: HashMap<String, String>,
    pub clickable: bool,
    pub other_forms: Vec<String>,
    pub length: usize,
    pub whitespace_after: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ParsedWords {
    pub sentences: Vec<String>,
    pub sections: Vec<Section>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TimeSpentPoint {
    pub name: String,
    pub duration: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NumWordsKnown {
    pub name: String,
    pub amount: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TimeSpentStats {
    pub days_this_week: Vec<TimeSpentPoint>,
    pub total_this_week: (String, String),
    pub this_month: (String, String),
    pub this_year: (String, String),
    pub total: (String, String),
    pub streak: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Debug)]
#[serde(tag = "t", content = "c")]
pub enum DictFileType {
    TextSplitAt(String),
    StarDict,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Dictionary {
    pub name: String,
    pub run_when_not: Option<String>,
    pub specific_settings: DictionarySpecificSettings,
    pub fetch_by_default: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "t", content = "c")]
pub enum DictionarySpecificSettings {
    File(String, DictFileType),
    Url(String),
    Command(String),
    EkalbaBendrines,
    EkalbaDabartines,
    Wiktionary(String, String),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Definition {
    Text(String),
    OnDemand(String),
    Empty,
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
    pub search_params: String,
}

#[derive(Deserialize, Serialize, Clone)]

pub struct Note(
    #[serde(with = "HashMapToArray::<String, NoteToWordHandling, NoteKeyValueLabels>")]
    pub  HashMap<String, NoteToWordHandling>,
);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SiteConfiguration {
    pub sites: Vec<String>,
    pub main_section: String,
    pub title_selector: String,
    pub subtitle_selector: String,
    pub image_selector: String,
    pub caption_selector: String,
    pub caption_separator: Option<String>,
    pub paragraph_selector: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Section {
    Title(Vec<Word>),
    Subtitle(Vec<Word>),
    Image(String),
    Caption(Vec<Word>),
    Paragraph(Vec<Word>),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    pub to_run: Option<Vec<String>>,
    pub dark_mode: bool,
    pub site_configurations: HashMap<String, SiteConfiguration>,
    pub languages: HashMap<String, LanguageSettings>,
    pub stanza_enabled: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LanguageSettings {
    pub deck: String,
    pub note_type: String,
    pub note_fields: HashMap<String, String>,
    pub model: String,
    pub dicts: Vec<Dictionary>,
    #[serde(with = "HashMapToArray::<String, Note, DeckKeyValueLabels>")]
    pub anki_parser: HashMap<String, Note>,
    pub frequency_list: String,
    pub words_known_by_freq: usize,
    pub grammar_parser: String,
    pub run_on_lemmas: Vec<String>,
    pub suggest_on_lemmas: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        let site_configurations = toml::from_str(include_str!("../../data/site_templates.toml"))
            .expect("Failed to parse site_templates.toml");
        Self {
            to_run: None,
            dark_mode: true,
            languages: HashMap::new(),
            site_configurations,
            stanza_enabled: false,
        }
    }
}

impl Default for LanguageSettings {
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
            anki_parser: HashMap::new(),
            frequency_list: String::from(""),
            words_known_by_freq: 0,
            grammar_parser: String::new(),
            suggest_on_lemmas: Vec::new(),
            run_on_lemmas: Vec::new(),
        }
    }
}
