use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    process,
    sync::Arc,
};

use crate::{
    spyglys_integration::{get_alternate_forms, handle_lemma, load_spyglys},
    LanguageParser, SakinyjeError, SakinyjeState, SharedInfo,
};
use log::{info, trace};
use lol_html::{element, text, RewriteStrSettings};
use shared::*;
use spyglys::interpreter::Interpreter;
use tauri::{State, Window};
use tokio::{
    runtime::{Handle, Runtime},
    sync::{Mutex, MutexGuard},
    task,
};
use url::Url;

#[tauri::command]
pub async fn parse_url(
    url: &str,
    state: State<'_, SakinyjeState>,
) -> Result<Vec<Section>, SakinyjeError> {
    let parsed_url = Url::parse(url).unwrap();
    let root_url = parsed_url.host_str().unwrap().strip_prefix("www.").unwrap();

    let site_config = {
        let locked_state = state.0.lock().await;
        info!("Root url: {}", root_url);
        trace!(
            "Site configurations: {:?}",
            locked_state.settings.site_configurations
        );
        locked_state
            .settings
            .site_configurations
            .get(root_url)
            .unwrap()
            .clone()
    };
    let response = reqwest::get(url)
        .await?
        .text()
        .await
        .expect("to get valid bytes");

    let sections = Arc::new(Mutex::new(Vec::new()));
    let state = Arc::new(state);

    let section_handlers = vec![
        text!(
            format!(
                "{} {}",
                site_config.main_section, site_config.title_selector
            ),
            |text| {
                if text.as_str().trim().is_empty() {
                    return Ok(());
                }
                let title_state = Arc::clone(&state);
                let title_sections = Arc::clone(&sections);
                let handle = Handle::current();
                task::block_in_place(|| {
                    handle.block_on(async move {
                        let mut sections = title_sections.lock().await;
                        sections.push(Section::Title(
                            words_from_string(text.as_str(), title_state).await?,
                        ));
                        Ok::<(), SakinyjeError>(())
                    })
                })?;
                Ok(())
            }
        ),
        text!(
            format!(
                "{} {}",
                site_config.main_section, site_config.subtitle_selector
            ),
            |text| {
                if text.as_str().trim().is_empty() {
                    return Ok(());
                }
                let subtitle_state = Arc::clone(&state);
                let subtitle_sections = Arc::clone(&sections);
                let handle = Handle::current();
                task::block_in_place(|| {
                    handle.block_on(async move {
                        let mut sections = subtitle_sections.lock().await;
                        sections.push(Section::Subtitle(
                            words_from_string(text.as_str(), subtitle_state).await?,
                        ));
                        Ok::<(), SakinyjeError>(())
                    })
                })?;
                Ok(())
            }
        ),
        text!(
            format!(
                "{} {}",
                site_config.main_section, site_config.caption_selector,
            ),
            |text| {
                if text.as_str().trim().is_empty() {
                    return Ok(());
                }
                log::info!("Caption text: {}", text.as_str());
                let caption_state = Arc::clone(&state);
                let caption_sections = Arc::clone(&sections);
                let handle = Handle::current();
                task::block_in_place(|| {
                    handle.block_on(async move {
                        let mut sections = caption_sections.lock().await;
                        sections.push(Section::Caption(
                            words_from_string(text.as_str(), caption_state).await?,
                        ));
                        Ok::<(), SakinyjeError>(())
                    })
                })?;
                Ok(())
            }
        ),
        text!(
            format!(
                "{} {}",
                site_config.main_section, site_config.paragraph_selector
            ),
            |text| {
                log::info!("Paragraph text: {}", text.as_str());
                if text.as_str().trim().is_empty() {
                    return Ok(());
                }
                let paragraph_state = Arc::clone(&state);
                let paragraph_sections = Arc::clone(&sections);
                let handle = Handle::current();
                task::block_in_place(|| {
                    handle.block_on(async move {
                        let mut sections = paragraph_sections.lock().await;
                        sections.push(Section::Paragraph(
                            words_from_string(text.as_str(), paragraph_state).await?,
                        ));
                        Ok::<(), SakinyjeError>(())
                    })
                })?;
                Ok(())
            }
        ),
        element!(
            format!(
                "{} {}",
                site_config.main_section, site_config.image_selector
            ),
            |el| {
                let image_sections = Arc::clone(&sections);
                let handle = Handle::current();
                task::block_in_place(|| {
                    handle.block_on(async move {
                        if let Some(src) = el.get_attribute("src") {
                            let mut sections = image_sections.lock().await;
                            sections.push(Section::Image(format!("https://www.{root_url}/{src}")));
                        }
                    })
                });
                Ok(())
            }
        ),
    ];
    lol_html::rewrite_str(
        &response,
        RewriteStrSettings {
            element_content_handlers: section_handlers,
            ..Default::default()
        },
    )
    .unwrap();
    let owned_sections = Arc::into_inner(sections).unwrap();
    Ok(owned_sections.into_inner())
}

#[tauri::command]
pub async fn parse_text(
    sent: &str,
    state: State<'_, SakinyjeState>,
) -> Result<Vec<Section>, SakinyjeError> {
    Ok(vec![Section::Paragraph(
        words_from_string(sent, Arc::new(state)).await?,
    )])
}

pub async fn words_from_string(
    sent: &str,
    state: Arc<State<'_, SakinyjeState>>,
) -> Result<Vec<Word>, SakinyjeError> {
    let mut state = state.0.lock().await;

    if sent.is_empty() {
        return Ok(Vec::new());
    }
    log::info!("Parsing text: {}", sent);

    let language = state
        .current_language
        .clone()
        .expect("Language to have already been chosen");
    let interpreter = load_spyglys(&mut state)?;

    let words = if state.language_parser.is_some() {
        log::trace!("Sending to stanza parser");
        stanza_parser(sent, &mut state, language.clone(), &interpreter)
    } else {
        default_tokenizer(sent, language.clone(), &mut state, &interpreter)
    }?;
    state
        .to_save
        .language_specific
        .get_mut(&language)
        .expect("language to have state")
        .words_seen
        .push((
            chrono::Utc::now(),
            words.iter().filter(|v| v.clickable).count(),
        ));
    Ok(words)
}

#[derive(serde::Deserialize, Clone)]
struct StanzaToken {
    text: String,
    lemma: String,
    upos: String,
    feats: Option<String>,
}

#[tauri::command]
pub async fn start_stanza(
    state: State<'_, SakinyjeState>,
    window: Window,
) -> Result<(), SakinyjeError> {
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
    window.emit(
        "stanza_loading",
        Some(ToasterPayload {
            message: Some(&format!("Loading model {model}")),
        }),
    )?;

    let mut buf = [0; 5];
    stdout.read_exact(&mut buf)?;
    if buf != "done\n".as_bytes() {
        panic!("Starting stanza failed {}", String::from_utf8_lossy(&buf))
    }
    log::info!("Stanza model loaded");
    window.emit("stanza_loading", Some(ToasterPayload { message: None }))?;

    state.language_parser = Some(LanguageParser { stdin, stdout });
    Ok(())
}

fn stanza_parser(
    sent: &str,
    state: &mut MutexGuard<SharedInfo>,
    language: String,
    interpreter: &Interpreter,
) -> Result<Vec<Word>, SakinyjeError> {
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
            let lemma = handle_lemma(&token.lemma, interpreter, state)?;
            let rating = state
                .to_save
                .language_specific
                .get_mut(&language)
                .expect("language to be chosen")
                .words
                .entry(lemma.clone())
                .or_insert(crate::WordInfo {
                    rating: 0,
                    method: crate::Method::FromSeen,
                    history: vec![(chrono::Utc::now(), crate::Method::FromSeen, 0)],
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

            Ok(Word {
                text: token.text,
                lemma: lemma.clone(),
                rating,
                morph,
                clickable: token.upos != "PUNCT",
                other_forms: get_alternate_forms(&lemma, interpreter, state)?,
            })
        })
        .collect::<Result<Vec<Word>, SakinyjeError>>()
}

fn default_tokenizer(
    sent: &str,
    language: String,
    state: &mut MutexGuard<SharedInfo>,
    interpreter: &Interpreter,
) -> Result<Vec<Word>, SakinyjeError> {
    let mut words = Vec::new();
    let mut currently_building = String::new();
    let mut chars = sent.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_alphabetic() {
            currently_building.push(c);
        } else {
            if !currently_building.is_empty() {
                let word = std::mem::take(&mut currently_building);
                // let lemma = handle_lemma(&token.lemma, interpreter)?;
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
                        history: vec![(chrono::Utc::now(), crate::Method::FromSeen, 0)],
                    })
                    .rating;
                words.push(Word {
                    text: word.clone(),
                    clickable: true,
                    lemma: word.clone(),
                    rating,
                    morph: HashMap::new(),
                    other_forms: get_alternate_forms(&word, interpreter, state)?,
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
                other_forms: Vec::new(),
            })
        }
    }
    Ok(words)
}
