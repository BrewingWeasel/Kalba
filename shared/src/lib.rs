use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Word {
    pub text: String,
    pub lemma: String,
    pub morph: Option<String>,
    pub clickable: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Debug)]
pub enum DictFileType {
    TextSplitAt(String),
    StarDict,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Dictionary {
    File(String, DictFileType),
    Url(String),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    pub deck: String,
    pub note_type: String,
    pub note_fields: String,
    pub model: String,
    pub dicts: Vec<Dictionary>,
    pub to_remove: Option<usize>,
}
