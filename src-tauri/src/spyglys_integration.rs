use spyglys::interpreter::{Interpreter, Value};
use tokio::sync::MutexGuard;

use crate::{SakinyjeError, SharedInfo};

pub fn handle_lemma(lemma: &str, interpreter: &Interpreter) -> Result<String, SakinyjeError> {
    let response = interpreter.run_function("update_lemma", lemma)?;
    match response {
        Value::Str(s) => Ok(s),
        Value::Empty => Ok(lemma.to_owned()),
        _ => todo!(),
    }
}

pub fn load_spyglys(state: &mut MutexGuard<SharedInfo>) -> Result<Interpreter, SakinyjeError> {
    let lang = state
        .current_language
        .as_ref()
        .expect("language to already be selected");
    let spyglys_grammar = &state
        .settings
        .languages
        .get(lang)
        .expect("language to exist")
        .grammar_parser;
    let interpreter = spyglys::contents_to_interpreter(spyglys_grammar)?;
    Ok(interpreter)
}
