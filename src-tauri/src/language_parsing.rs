use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    process,
    sync::Arc,
};

use crate::{
    commands::new_command,
    spyglys_integration::{get_alternate_forms, handle_lemma, load_spyglys},
    KalbaError, KalbaState, LanguageParser, SharedInfo,
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
pub async fn get_url_contents(url: &str) -> Result<String, KalbaError> {
    Ok(reqwest::get(url)
        .await?
        .text()
        .await
        .expect("to get valid bytes"))
}

struct SectionDetails {
    sections: Vec<SectionContents>,
    text: String,
    last_subtitle: Option<String>,
    last_link: Option<String>,
    was_just_link: bool,
}

#[tauri::command]
pub async fn parse_url(
    url: &str,
    contents: &str,
    title: &str,
    state: State<'_, KalbaState>,
) -> Result<ParsedWords, KalbaError> {
    let parsed_url = Url::parse(url).unwrap();
    let url = parsed_url.host_str().unwrap();
    let root_url = url.strip_prefix("www.").unwrap_or(url);

    let site_config = {
        let locked_state = state.0.lock().await;
        info!("Root url: {}", root_url);
        trace!(
            "Site configurations: {:?}",
            locked_state.settings.site_configurations
        );
        let mut site_config = None;
        for possible_site in locked_state.settings.site_configurations.values() {
            if possible_site.sites.contains(&root_url.to_owned()) {
                site_config = Some(possible_site.to_owned());
                break;
            }
        }
        site_config
    };

    let sections = Arc::new(Mutex::new(SectionDetails {
        sections: vec![SectionContents::Title(title.chars().count())],
        text: format!("{title}\n"),
        last_subtitle: None,
        last_link: None,
        was_just_link: false,
    }));
    let state = Arc::new(state);

    let section_handlers = vec![
        text!("h1", |text| {
            if text.as_str().trim().is_empty() {
                return Ok(());
            }
            let title_sections = Arc::clone(&sections);
            let handle = Handle::current();
            task::block_in_place(|| {
                handle.block_on(async move {
                    let mut sections = title_sections.lock().await;
                    sections.text.push_str(text.as_str());
                    sections.text.push('\n');
                    sections.sections.push(SectionContents::Title(
                        text.as_str().trim_start().chars().count(),
                    ));
                    Ok::<(), KalbaError>(())
                })
            })?;
            Ok(())
        }),
        text!("p > strong, h2 > strong", |text| {
            if text.as_str().trim().is_empty()
                || site_config
                    .as_ref()
                    .is_some_and(|v| v.ignore_strings.contains(&text.as_str().to_owned()))
            {
                return Ok(());
            }
            log::info!("Subtitle text: {}", text.as_str());
            let subtitle_sections = Arc::clone(&sections);
            let handle = Handle::current();
            task::block_in_place(|| {
                handle.block_on(async move {
                    let mut section_details = subtitle_sections.lock().await;
                    section_details.text.push_str(text.as_str());
                    section_details.text.push('\n');
                    section_details.sections.push(SectionContents::Subtitle(
                        text.as_str().trim_start().chars().count(),
                    ));
                    section_details.last_subtitle = Some(text.as_str().to_owned());
                })
            });
            Ok(())
        }),
        text!("figcaption p", |text| {
            if text.as_str().trim().is_empty() {
                return Ok(());
            }
            let text = if let Some(separator) = site_config
                .as_ref()
                .and_then(|c| c.caption_separator.as_ref())
            {
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
                    section_details.text.push_str(text);
                    section_details.text.push('\n');
                    section_details
                        .sections
                        .push(SectionContents::Caption(text.chars().count()));
                })
            });
            Ok(())
        }),
        text!("p > a", |text| {
            if text.as_str().trim().is_empty()
                || site_config
                    .as_ref()
                    .is_some_and(|v| v.ignore_strings.contains(&text.as_str().to_owned()))
            {
                return Ok(());
            }
            let paragraph_sections = Arc::clone(&sections);
            let handle = Handle::current();
            task::block_in_place(|| {
                handle.block_on(async move {
                    let mut section_details = paragraph_sections.lock().await;
                    log::info!("Found link {}", text.as_str());
                    section_details.text = section_details.text.trim_end().to_owned();
                    section_details.text.push(' ');
                    section_details.text.push_str(text.as_str());
                    if let Some(SectionContents::Paragraph(v)) = section_details.sections.last_mut()
                    {
                        *v += text.as_str().trim_start().chars().count() + 1;
                    }
                    section_details.last_subtitle = Some(text.as_str().to_owned());
                })
            });
            Ok(())
        }),
        text!("p", |text| {
            if text.as_str().trim().is_empty()
                || site_config
                    .as_ref()
                    .is_some_and(|v| v.ignore_strings.contains(&text.as_str().to_owned()))
            {
                return Ok(());
            }
            let paragraph_sections = Arc::clone(&sections);
            let handle = Handle::current();
            task::block_in_place(|| {
                handle.block_on(async move {
                    let mut section_details = paragraph_sections.lock().await;
                    if let Some(last) = std::mem::take(&mut section_details.last_subtitle) {
                        if last == text.as_str() {
                            return;
                        }
                    }

                    if section_details
                        .last_link
                        .as_ref()
                        .is_some_and(|l| l == text.as_str())
                    {
                        section_details.last_link = None;
                        if text.last_in_text_node() {
                            section_details.text.push('\n');
                        } else {
                            section_details.was_just_link = true;
                            section_details.text.push(' ');
                        }
                        return;
                    }

                    section_details.text.push_str(text.as_str());
                    section_details.text.push('\n');
                    if section_details.was_just_link {
                        section_details.was_just_link = false;
                        if let Some(SectionContents::Paragraph(v)) =
                            section_details.sections.last_mut()
                        {
                            *v += text.as_str().trim_start().chars().count();
                        }
                    } else {
                        section_details.sections.push(SectionContents::Paragraph(
                            text.as_str().trim_start().chars().count(),
                        ));
                    }
                })
            });
            Ok(())
        }),
        element!("img", |el| {
            let image_sections = Arc::clone(&sections);
            let handle = Handle::current();
            task::block_in_place(|| {
                handle.block_on(async move {
                    if let Some(src) = el.get_attribute("src") {
                        let sections = &mut image_sections.lock().await.sections;
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
        }),
    ];

    lol_html::rewrite_str(
        contents,
        RewriteStrSettings {
            element_content_handlers: section_handlers,
            ..Default::default()
        },
    )
    .unwrap();
    log::info!("Created sections");

    let owned_sections = Arc::into_inner(sections).unwrap();
    let owned_details = owned_sections.into_inner();
    let (sentences, all_words) = words_from_string(&owned_details.text, state).await?;

    let mut all_words = all_words.into_iter().peekable();

    let mut get_words = |length| {
        log::trace!("section length: {length}");
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
    for section_content in owned_details.sections {
        let section = match section_content {
            SectionContents::Paragraph(length) => Section::Paragraph(get_words(length)),
            SectionContents::Title(length) => Section::Title(get_words(length)),
            SectionContents::Subtitle(length) => Section::Subtitle(get_words(length)),
            SectionContents::Caption(length) => Section::Caption(get_words(length)),
            SectionContents::SpecificSection(s) => s,
        };
        sections.push(section);
    }

    Ok(ParsedWords {
        sentences,
        sections,
    })
}

#[tauri::command]
pub async fn parse_text(
    sent: &str,
    state: State<'_, KalbaState>,
) -> Result<ParsedWords, KalbaError> {
    let (sentences, words) = words_from_string(sent, Arc::new(state)).await?;
    Ok(ParsedWords {
        sentences,
        sections: vec![Section::Paragraph(words)],
    })
}

pub async fn words_from_string(
    sent: &str,
    state: Arc<State<'_, KalbaState>>,
) -> Result<(Vec<String>, Vec<Word>), KalbaError> {
    let mut state = state.0.lock().await;

    if sent.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }
    log::info!("Parsing text: {}", sent);

    let language = state
        .current_language
        .clone()
        .expect("Language to have already been chosen");
    let interpreter = load_spyglys(&mut state)?;

    let (sentences, words) = if state.language_parser.is_some() && state.settings.stanza_enabled {
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
    Ok((sentences, words))
}

#[derive(serde::Deserialize, Clone)]
struct Sentence {
    words: Vec<StanzaToken>,
    sentence: String,
}

#[derive(serde::Deserialize, Clone)]
struct StanzaToken {
    text: String,
    lemma: String,
    upos: String,
    feats: Option<String>,
    // For some ungodly reason, these are not included with mwt (at least for spanish)
    // in these cases, we have to calculate them ourselves based on the previous words
    start_char: Option<usize>,
    end_char: Option<usize>,
}

#[tauri::command]
pub async fn start_stanza(state: State<'_, KalbaState>, window: Window) -> Result<(), KalbaError> {
    let mut state = state.0.lock().await;
    if state.language_parser.is_some() || !state.settings.stanza_enabled {
        return Ok(());
    }

    let stanza_path = dirs::data_dir()
        .ok_or_else(|| KalbaError::MissingDir("data".to_owned()))?
        .join("kalba")
        .join("stanza");
    let mut process = new_command(
        stanza_path
            .join(".venv")
            .join(if cfg!(target_os = "windows") {
                "Scripts"
            } else {
                "bin"
            })
            .join("python"),
    )
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

    let model_exists = stanza_path.join("stanza_models").join(model).exists();

    let model_formatted = format!("{model}\n");
    let bytes_written = stdin.write(model_formatted.as_bytes())?;
    if bytes_written != model_formatted.as_bytes().len() {
        return Err(KalbaError::IncorrectWrite(model_formatted, bytes_written));
    }
    log::info!("Loading stanza model {model} for language {language}");
    window.emit(
        "stanza_loading",
        Some(ToasterPayload {
            message: Some(&if model_exists {
                format!("Loading model {model}")
            } else {
                format!("Installing model {model} (this may take a minute)")
            }),
        }),
    )?;

    let mut buf = String::new();
    stdout.read_line(&mut buf)?;
    if buf.trim_end() != "done" {
        panic!("Starting stanza failed {buf}");
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
) -> Result<(Vec<String>, Vec<Word>), KalbaError> {
    let language_parser = state
        .language_parser
        .as_mut()
        .expect("language parser to be started");

    let sent_formatted = format!("{}\n", sent.replace("\n\n", "\n"));
    let bytes_written = language_parser
        .stdin
        .write(sent_formatted.as_bytes())
        .expect("to write to stdin");
    if bytes_written != sent_formatted.as_bytes().len() {
        return Err(KalbaError::IncorrectWrite(sent_formatted, bytes_written));
    }

    log::trace!("sentence written");

    let mut contents = String::new();
    loop {
        let mut specific_contents = String::new();
        if language_parser
            .stdout
            .read_line(&mut specific_contents)
            .is_err()
            || specific_contents.trim_end() == "done"
        {
            break;
        }
        contents.push_str(&specific_contents);
    }
    let details =
        serde_json::from_str::<Vec<Sentence>>(&contents).expect("valid json from stanza parser");
    log::trace!("response parsed");

    let mut words = Vec::new();
    let mut sentences = Vec::new();

    for (sentence_index, sentence) in details.into_iter().enumerate() {
        sentences.push(sentence.sentence);
        let mut tokens = sentence.words.into_iter().peekable();
        let mut last_end = 0;
        while let Some(token) = tokens.next() {
            let lemma = handle_lemma(&token.lemma, interpreter, state)?;
            let rating = if ["PUNCT", "SYM", "NUM"].contains(&token.upos.as_str()) {
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
                        rating: if token.upos == "PROPN" { -1 } else { 0 },
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

            let mut end_char = token.end_char.unwrap_or(sent.len());
            let start_char = token.start_char.unwrap_or(last_end);
            let mut text = token.text;

            // mwt
            if token.end_char.is_none() {
                while let Some(next_token) = tokens.peek() {
                    if let Some(next_start) = next_token.start_char {
                        end_char = next_start;
                        break;
                    }
                    tokens.next();
                }
                text = sent
                    .chars()
                    .skip(start_char)
                    .take(end_char - start_char)
                    .collect();
            }

            let whitespace_after = if token.end_char.is_some() {
                if let Some(next_start) = tokens.peek().and_then(|t| t.start_char) {
                    next_start != end_char
                } else {
                    true
                }
            } else if text.trim_end().chars().count() != text.chars().count() {
                text = text.trim_end().to_owned();
                end_char -= 1;
                true
            } else {
                false
            };

            last_end = end_char;

            words.push(Word {
                text,
                lemma: lemma.clone(),
                rating,
                morph,
                sentence_index,
                clickable: !["PUNCT", "SYM", "NUM"].contains(&token.upos.as_str()),
                other_forms: get_alternate_forms(&lemma, interpreter, state)?,
                length: end_char - start_char,
                whitespace_after,
            })
        }
    }
    Ok((sentences, words))
}

fn default_tokenizer(
    sent: &str,
    language: String,
    state: &mut MutexGuard<SharedInfo>,
    interpreter: &Interpreter,
) -> Result<(Vec<String>, Vec<Word>), KalbaError> {
    let mut words = Vec::new();
    let mut sentences = Vec::new();
    if sent.is_empty() {
        return Ok((sentences, words));
    }
    let mut current_sentence = String::new();

    let mut currently_building = String::new();
    let mut chars = sent.chars().peekable();
    while let Some(c) = chars.next() {
        current_sentence.push(c);
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
                    length: word.chars().count(),
                    whitespace_after,
                    sentence_index: sentences.len(),
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
                sentence_index: sentences.len(),
            });

            if ['.', '!', '?'].contains(&c) {
                sentences.push(std::mem::take(&mut current_sentence));
            }
        }
    }

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
            length: word.chars().count(),
            whitespace_after: true,
            sentence_index: sentences.len(),
        })
    }
    if !current_sentence.is_empty() {
        sentences.push(current_sentence);
    }
    Ok((sentences, words))
}
