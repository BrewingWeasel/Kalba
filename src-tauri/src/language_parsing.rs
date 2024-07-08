use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    process,
    time::Duration,
};

use crate::{LanguageParser, SakinyjeError, SakinyjeState, SharedInfo};
use shared::*;
use tauri::State;
use tokio::{sync::MutexGuard, time::sleep};

#[tauri::command]
pub async fn parse_text(sent: &str, state: State<'_, SakinyjeState>) -> Result<Vec<Word>, String> {
    if sent.is_empty() {
        return Ok(Vec::new());
    }
    log::info!("Parsing text: {}", sent);

    let mut state = state.0.lock().await;
    let language = state
        .current_language
        .clone()
        .expect("Language to have already been chosen");

    if state.language_parser.is_some() {
        log::trace!("Sending to stanza parser");
        Ok(stanza_parser(sent, &mut state, language))
    } else {
        Ok(default_tokenizer(sent, language, &mut state))
    }
}
#[derive(serde::Deserialize, Clone)]
struct StanzaToken {
    text: String,
    lemma: String,
    upos: String,
    feats: Option<String>,
}

#[tauri::command]
pub async fn start_stanza(state: State<'_, SakinyjeState>) -> Result<(), SakinyjeError> {
    let mut state = state.0.lock().await;
    if state.language_parser.is_some() {
        return Ok(());
    }

    let stanza_path = dirs::data_dir()
        .ok_or_else(|| SakinyjeError::MissingDir("data".to_owned()))?
        .join("sakinyje")
        .join("stanza");
    let mut process = process::Command::new(stanza_path.join(".venv").join("bin").join("python3"))
        .arg(stanza_path.join("run.py"))
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;
    log::info!("Started stanza");

    let language = state
        .current_language
        .as_ref()
        .expect("language to be chosen");
    let model = &state
        .settings
        .languages
        .get(language)
        .expect("language chosen to exist")
        .model;

    let mut stdout =
        BufReader::new(std::mem::take(&mut process.stdout).expect("stdout to be piped"));
    let mut stdin = std::mem::take(&mut process.stdin).expect("stdin to be piped");

    stdin.write(format!("{model}\n").as_bytes())?;
    log::info!("Loading stanza model {model} for language {language}");
    let mut buf = [0; 5];
    stdout.read_exact(&mut buf)?;
    if buf != "done\n".as_bytes() {
        panic!("Starting stanza failed {}", String::from_utf8_lossy(&buf))
    }
    log::info!("Stanza model loaded");

    state.language_parser = Some(LanguageParser { stdin, stdout });
    Ok(())
}

fn stanza_parser(sent: &str, state: &mut MutexGuard<SharedInfo>, language: String) -> Vec<Word> {
    let language_parser = state
        .language_parser
        .as_mut()
        .expect("language parser to be started");
    language_parser
        .stdin
        .write(format!("{sent}\n").as_bytes())
        .expect("to write to stdin");
    log::trace!("sentence written");
    let mut contents = String::new();
    loop {
        let mut specific_contents = String::new();
        if language_parser
            .stdout
            .read_line(&mut specific_contents)
            .is_err()
        {
            break;
        }
        contents.push_str(&specific_contents);
        if specific_contents == "]\n" {
            break;
        }
    }
    let details = serde_json::from_str::<Vec<StanzaToken>>(&contents).unwrap();
    log::trace!("response parsed");
    details
        .into_iter()
        .map(|token| {
            let rating = state
                .to_save
                .language_specific
                .get_mut(&language)
                .expect("language to be chosen")
                .words
                .entry(token.lemma.clone())
                .or_insert(crate::WordInfo {
                    rating: 0,
                    method: crate::Method::FromSeen,
                })
                .rating;

            let morph = token
                .feats
                .map(|feats| {
                    feats
                        .split('|')
                        .map(|morph| {
                            let mut morph_parts = morph.split('=');
                            let key = morph_parts.next().unwrap().to_string();
                            let value = morph_parts.next().unwrap().to_string();
                            (key, value)
                        })
                        .collect()
                })
                .unwrap_or_default();

            Word {
                text: token.text,
                lemma: token.lemma,
                rating,
                morph,
                clickable: token.upos != "PUNCT",
            }
        })
        .collect()
}

fn default_tokenizer(
    sent: &str,
    language: String,
    state: &mut MutexGuard<SharedInfo>,
) -> Vec<Word> {
    let mut words = Vec::new();
    let mut currently_building = String::new();
    let mut chars = sent.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_alphabetic() {
            currently_building.push(c);
        } else {
            if !currently_building.is_empty() {
                let word = std::mem::take(&mut currently_building);
                let rating = state
                    .to_save
                    .language_specific
                    .get_mut(&language)
                    .expect("language to be chosen")
                    .words
                    .entry(word.clone())
                    .or_insert(crate::WordInfo {
                        rating: 0,
                        method: crate::Method::FromSeen,
                    })
                    .rating;
                words.push(Word {
                    text: word.clone(),
                    clickable: true,
                    lemma: word,
                    rating,
                    morph: HashMap::new(),
                })
            }
            while let Some(possible_whitespace) = chars.peek() {
                if possible_whitespace.is_whitespace() {
                    chars.next();
                } else {
                    break;
                }
            }
            if c.is_whitespace() {
                continue;
            }
            words.push(Word {
                text: c.to_string(),
                clickable: false,
                lemma: c.to_string(),
                rating: 4,
                morph: HashMap::new(),
            })
        }
    }
    words
}
