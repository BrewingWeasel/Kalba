use std::error::Error;

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

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "t", content = "conts")]
pub enum SakinyjeResult<T> {
    Ok(T),
    Err(String),
}

impl<T> From<Result<T, Box<dyn Error>>> for SakinyjeResult<T> {
    fn from(value: Result<T, Box<dyn Error>>) -> Self {
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
pub struct Settings {
    pub deck: String,
    pub note_type: String,
    pub note_fields: String,
    pub model: String,
    pub dicts: Vec<Dictionary>,
    pub to_remove: Option<usize>,
    pub css: Option<String>,
}
