use pyo3::prelude::*;
use std::{collections::HashMap, str::FromStr};

pub struct Token {
    pub text: String,
    pub lemma: String,
    pub pos: PartOfSpeech,
    pub morph: HashMap<String, String>,
}

pub enum PartOfSpeech {
    Adjective,
    Adposition,
    Adverb,
    Auxiliary,
    CoordinatingConjunction,
    Determiner,
    Interjection,
    Noun,
    Numeral,
    Particle,
    Pronoun,
    ProperNoun,
    Punctuation,
    SubordinatingConjunction,
    Symbol,
    Verb,
    Other,
}

impl FromStr for PartOfSpeech {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADJ" => Ok(Self::Adjective),
            "ADP" => Ok(Self::Adposition),
            "ADV" => Ok(Self::Adverb),
            "AUX" => Ok(Self::Auxiliary),
            "CCONJ" => Ok(Self::CoordinatingConjunction),
            "DET" => Ok(Self::Determiner),
            "INTJ" => Ok(Self::Interjection),
            "NOUN" => Ok(Self::Noun),
            "NUM" => Ok(Self::Numeral),
            "PART" => Ok(Self::Particle),
            "PRON" => Ok(Self::Pronoun),
            "PROPN" => Ok(Self::ProperNoun),
            "PUNCT" => Ok(Self::Punctuation),
            "SCONJ" => Ok(Self::SubordinatingConjunction),
            "SYM" => Ok(Self::Symbol),
            "VERB" => Ok(Self::Verb),
            "X:" => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

pub fn get_spacy_model(model: &str) -> Result<Py<PyAny>, String> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let spacy = PyModule::import(py, "spacy")?;
        let v = spacy.getattr("load")?.call1((model,))?;
        Ok(v.to_object(py))
    })
    .map_err(|e| e.to_string())
}

pub fn get_spacy_info(sent: &str, morphologizer: &PyObject) -> Result<Vec<Token>, String> {
    Python::with_gil(|py| -> PyResult<Vec<Token>> {
        let mut words = Vec::new();
        // let spacy = PyModule::import(py, "spacy")?;
        // let morphologizerr = match spacy.getattr("load")?.call1((model,)) {
        //     Ok(v) => v,
        //     Err(_) => {
        //         return Err(PyEnvironmentError::new_err(format!(
        //             "Unable to load {model}"
        //         )))
        //     }
        // };

        let total: Vec<PyObject> = morphologizer.call1(py, (sent,))?.extract(py)?;
        for token in total {
            let text: String = token.getattr(py, "text")?.extract(py)?;
            let pos_str: String = token.getattr(py, "pos_")?.extract(py)?;
            let pos = if let Ok(p) = PartOfSpeech::from_str(&pos_str) {
                p
            } else {
                PartOfSpeech::Other
            };
            let lemma: String = token.getattr(py, "lemma_")?.extract(py)?;
            let morph: HashMap<String, String> = token
                .getattr(py, "morph")?
                .getattr(py, "to_dict")?
                .call0(py)?
                .extract(py)?;

            words.push(Token {
                text,
                lemma,
                pos,
                morph,
            })
        }
        Ok(words)
    })
    .map_err(|e| e.to_string())
}
