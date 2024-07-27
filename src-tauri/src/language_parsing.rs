use std::{
    collections::{HashMap, HashSet},
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
    runtime::Handle,
    sync::{Mutex, MutexGuard},
    task,
};
use url::Url;

#[derive(Debug, Clone)]
enum SectionContents {
    Title(usize),
    Subtitle(usize),
    Caption(usize),
    Paragraph(usize),
    SpecificSection(Section),
}

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
        let mut site_config = Err(SakinyjeError::MissingSiteConfig(root_url.to_owned()));
        for possible_site in locked_state.settings.site_configurations.values() {
            if possible_site.sites.contains(&root_url.to_owned()) {
                site_config = Ok(possible_site.to_owned());
                break;
            }
        }
        site_config
    }?;

    let response = reqwest::get(url)
        .await?
        .text()
        .await
        .expect("to get valid bytes");

    let sections = Arc::new(Mutex::new((HashSet::new(), Vec::new(), String::new())));
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
                let title_sections = Arc::clone(&sections);
                let handle = Handle::current();
                task::block_in_place(|| {
                    handle.block_on(async move {
                        let mut sections = title_sections.lock().await;
                        sections.2.push_str(text.as_str());
                        sections.2.push('\n');
                        sections.1.push(SectionContents::Title(
                            text.as_str().trim_start().chars().count(),
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
                log::info!("Subtitle text: {}", text.as_str());
                let subtitle_sections = Arc::clone(&sections);
                let handle = Handle::current();
                task::block_in_place(|| {
                    handle.block_on(async move {
                        let mut section_details = subtitle_sections.lock().await;
                        section_details.0.insert(text.as_str().to_owned());
                        section_details.2.push_str(text.as_str());
                        section_details.2.push('\n');
                        section_details.1.push(SectionContents::Subtitle(
                            text.as_str().trim_start().chars().count(),
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
                let text = if let Some(separator) = site_config.caption_separator.as_ref() {
                    if let Some((main_caption, _)) = text.as_str().split_once(separator) {
                        main_caption.trim()
                    } else {
                        text.as_str()
                    }
                } else {
                    text.as_str()
                };
                let sections = Arc::clone(&sections);
                let handle = Handle::current();
                task::block_in_place(|| {
                    handle.block_on(async move {
                        let mut section_details = sections.lock().await;
                        section_details.2.push_str(text);
                        section_details.2.push('\n');
                        section_details
                            .1
                            .push(SectionContents::Caption(text.chars().count()));
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
                if text.as_str().trim().is_empty() {
                    return Ok(());
                }
                let paragraph_sections = Arc::clone(&sections);
                let handle = Handle::current();
                task::block_in_place(|| {
                    handle.block_on(async move {
                        let mut section_details = paragraph_sections.lock().await;
                        if section_details.0.contains(text.as_str()) {
                            return Ok::<(), SakinyjeError>(());
                        }
                        section_details.2.push_str(text.as_str());
                        section_details.2.push('\n');
                        section_details.1.push(SectionContents::Paragraph(
                            text.as_str().trim_start().chars().count(),
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
                            let sections = &mut image_sections.lock().await.1;
                            sections.push(SectionContents::SpecificSection(Section::Image(
                                if src.starts_with("http") {
                                    src.to_owned()
                                } else {
                                    format!("https://www.{root_url}/{src}")
                                },
                            )));
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
    log::info!("Created sections");

    let owned_sections = Arc::into_inner(sections).unwrap();
    let owned_details = owned_sections.into_inner();
    let mut all_words = words_from_string(&owned_details.2, state)
        .await?
        .into_iter()
        .peekable();

    let mut get_words = |length| {
        let mut current_length = 0;
        let mut words = Vec::new();
        while let Some(word) = all_words.peek() {
            log::trace!("word: {:?}", word);
            current_length += word.length;
            if word.whitespace_after {
                current_length += 1;
            }

            if current_length - 1 > length {
                break;
            }
            words.push(all_words.next().expect("already peeked"));
        }
        words
    };

    let mut sections = Vec::new();
    for section_content in owned_details.1 {
        let section = match section_content {
            SectionContents::Paragraph(length) => Section::Paragraph(get_words(length)),
            SectionContents::Title(length) => Section::Title(get_words(length)),
            SectionContents::Subtitle(length) => Section::Subtitle(get_words(length)),
            SectionContents::Caption(length) => Section::Caption(get_words(length)),
            SectionContents::SpecificSection(s) => s,
        };
        sections.push(section);
    }

    Ok(sections)
}

#[tauri::command]
pub async fn parse_text(
    sent: &str,
    state: State<'_, SakinyjeState>,
) -> Result<Vec<Section>, SakinyjeError> {
    Ok(vec![Section::Paragraph(dbg!(
        words_from_string(sent, Arc::new(state)).await?
    ))])
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

    let words = if state.language_parser.is_some() && state.settings.stanza_enabled {
        log::trace!("Sending to stanza parser");
        stanza_parser(
            &format!("{sent}\n"),
            &mut state,
            language.clone(),
            &interpreter,
        )
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
    start_char: usize,
    end_char: usize,
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

    let model_formatted = format!("{model}\n");
    let bytes_written = stdin.write(model_formatted.as_bytes())?;
    if bytes_written != model_formatted.as_bytes().len() {
        return Err(SakinyjeError::IncorrectWrite(
            model_formatted,
            bytes_written,
        ));
    }
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

    let sent_formatted = format!("{sent}\n");
    let bytes_written = language_parser
        .stdin
        .write(sent_formatted.as_bytes())
        .expect("to write to stdin");
    if bytes_written != sent_formatted.as_bytes().len() {
        return Err(SakinyjeError::IncorrectWrite(sent_formatted, bytes_written));
    }

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
    let mut details = serde_json::from_str::<Vec<StanzaToken>>(&contents)
        .unwrap()
        .into_iter()
        .peekable();
    log::trace!("response parsed");

    let mut words = Vec::new();

    while let Some(token) = details.next() {
        let lemma = handle_lemma(&token.lemma, interpreter, state)?;
        let rating = if ["PUNCT", "SYM", "PROPN", "NUM"].contains(&token.upos.as_str()) {
            -1
        } else {
            state
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
                .rating
        };

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

        let whitespace_after = if let Some(next_token) = details.peek() {
            next_token.start_char != token.end_char
        } else {
            false
        };

        words.push(Word {
            text: token.text,
            lemma: lemma.clone(),
            rating,
            morph,
            clickable: !["PUNCT", "SYM", "NUM"].contains(&token.upos.as_str()),
            other_forms: get_alternate_forms(&lemma, interpreter, state)?,
            length: token.end_char - token.start_char,
            whitespace_after,
        })
    }
    Ok(words)
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

                let whitespace_after = c.is_whitespace();

                words.push(Word {
                    text: word.clone(),
                    clickable: true,
                    lemma: word.clone(),
                    rating,
                    morph: HashMap::new(),
                    other_forms: get_alternate_forms(&word, interpreter, state)?,
                    length: word.len() + 1,
                    whitespace_after,
                })
            }
            let mut whitespace_after = false;
            while let Some(possible_whitespace) = chars.peek() {
                if possible_whitespace.is_whitespace() {
                    whitespace_after = true;
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
                rating: -1,
                morph: HashMap::new(),
                other_forms: Vec::new(),
                length: 1,
                whitespace_after,
            })
        }
    }
    Ok(words)
}
