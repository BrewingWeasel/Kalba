use pyo3::prelude::*;

pub struct Word {
    pub text: String,
    pub lemma: String,
    pub morph: Option<String>,
    pub clickable: bool,
}

pub fn get_words(sent: &str) -> Vec<Word> {
    match Python::with_gil(|py| -> PyResult<Vec<Word>> {
        let mut words = Vec::new();
        let spacy = PyModule::import(py, "spacy")?;
        let morphologizer = spacy.getattr("load")?.call1(("lt_core_news_sm",))?;
        let total: Vec<PyObject> = morphologizer.call1((sent,))?.extract()?;
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

            println!("{:?}", morph);
            words.push(Word {
                text,
                lemma,
                morph,
                clickable,
            })
        }
        Ok(words)
    }) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}
