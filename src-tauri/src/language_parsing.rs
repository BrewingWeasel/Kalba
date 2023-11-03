use pyo3::{exceptions::PyEnvironmentError, prelude::*};
use shared::*;
use tauri::State;

use crate::SakinyjeState;

#[tauri::command]
pub fn parse_text(
    state: State<'_, SakinyjeState>,
    sent: &str,
    model: &str,
) -> Result<Vec<Word>, String> {
    Python::with_gil(|py| -> PyResult<Vec<Word>> {
        let mut words = Vec::new();
        let spacy = PyModule::import(py, "spacy")?;
        let morphologizer = match spacy.getattr("load")?.call1((model,)) {
            Ok(v) => v,
            Err(_) => {
                return Err(PyEnvironmentError::new_err(format!(
                    "Unable to load {model}"
                )))
            }
        };
        let total: Vec<PyObject> = morphologizer.call1((sent,))?.extract()?;
        let mut state = state.0.lock().unwrap();
        for i in total {
            let text: String = i.getattr(py, "text")?.extract(py)?;
            let pos: String = i.getattr(py, "pos_")?.extract(py)?;
            let clickable = pos != "PUNCT";
            let lemma: String = i.getattr(py, "lemma_")?.extract(py)?;
            let morph: Option<String> = match i
                .getattr(py, "morph")
                .and_then(|v| v.getattr(py, "get")?.call1(py, ("Case",)))
                .and_then(|v| v.extract::<Vec<String>>(py))
            {
                Ok(mut s) if !s.is_empty() => Some(s.remove(0)),
                _ => None,
            };

            let rating = state
                .words
                .entry(lemma.clone())
                .or_insert(crate::WordInfo {
                    rating: 0,
                    can_modify: true,
                })
                .rating;

            println!("{:?}", morph);
            words.push(Word {
                text,
                lemma,
                rating,
                morph,
                clickable,
            })
        }
        Ok(words)
    })
    .map_err(|e| e.to_string())
}
