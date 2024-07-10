use spyglys::interpreter::{Interpreter, Value};
use tokio::sync::MutexGuard;

use crate::{SakinyjeError, SharedInfo};

pub fn handle_lemma(
    lemma: &str,
    interpreter: &Interpreter,
    state: &mut MutexGuard<SharedInfo>,
) -> Result<String, SakinyjeError> {
    let language = state
        .current_language
        .as_ref()
        .expect("language to already be selected");
    for modifier in &state
        .settings
        .languages
        .get(language)
        .expect("language to exist")
        .run_on_lemmas
    {
        let response = interpreter.run_function(modifier, lemma)?;
        log::trace!("Response: {:?} with function {modifier}", response);
        if let Value::Str(s) = response {
            if s != lemma {
                return Ok(s);
            }
        }
    }
    Ok(lemma.to_string())
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
