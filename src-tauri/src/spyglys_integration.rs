use spyglys::interpreter::{Interpreter, RuntimeError, Value};
use tauri::State;
use tokio::sync::MutexGuard;

use crate::{SakinyjeError, SakinyjeState, SharedInfo};

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

pub fn get_alternate_forms(
    lemma: &str,
    interpreter: &Interpreter,
    state: &mut MutexGuard<SharedInfo>,
) -> Result<Vec<String>, SakinyjeError> {
    let language = state
        .current_language
        .as_ref()
        .expect("language to already be selected");
    let mut responses = state
        .settings
        .languages
        .get(language)
        .expect("language to exist")
        .suggest_on_lemmas
        .iter()
        .filter_map(|modifier| {
            let response = match interpreter.run_function(modifier, lemma) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };
            log::trace!("Suggestion: {:?} with function {modifier}", response);
            if let Value::Str(s) = response {
                if s != lemma {
                    return Some(Ok(s));
                }
            }
            None
        })
        .collect::<Result<Vec<String>, RuntimeError>>()?;
    responses.sort();
    responses.dedup();
    Ok(responses)
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

#[tauri::command]
pub async fn get_spyglys_functions(
    state: State<'_, SakinyjeState>,
) -> Result<Vec<String>, SakinyjeError> {
    let current_interpreter = load_spyglys(&mut state.0.lock().await)?;
    Ok(current_interpreter.get_functions())
}

#[tauri::command]
pub async fn format_spyglys(state: State<'_, SakinyjeState>) -> Result<String, SakinyjeError> {
    let state = state.0.lock().await;
    let current_language = state
        .current_language
        .as_ref()
        .expect("language to already be selected");
    let parsed = spyglys::parse_string(
        &state
            .settings
            .languages
            .get(current_language)
            .expect("language to exist")
            .grammar_parser,
    )?;

    Ok(spyglys::formatter::pretty_file(&parsed))
}
