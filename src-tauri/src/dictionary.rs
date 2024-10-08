use lol_html::{element, html_content::ContentType, rewrite_str, text, RewriteStrSettings};
use reqwest::{header::USER_AGENT, Client};
use select::{
    document::Document,
    predicate::{self, Attr},
};
use serde::{Deserialize, Serialize};
use shared::{Definition, DefinitionStyling, DictFileType, DictionarySpecificSettings};
use std::{collections::HashMap, fs, sync::Arc};
use tauri::State;

use crate::{commands::run_command, KalbaError, KalbaState};

#[derive(Default, Clone)]
pub struct DictionaryInfo {
    client: Option<Client>,
    bendrines_file: Option<String>,
    dabartines_file: Option<String>,
    ekalba_bendrines: Option<HashMap<String, String>>,
    ekalba_dabartines: Option<HashMap<String, String>>,
}

pub enum EkalbaDictionary {
    Bendrines,
    Dabartines,
}

impl DictionaryInfo {
    async fn send_request(&mut self, url: &str) -> reqwest::Response {
        self.client
            .get_or_insert_with(Client::new)
            .get(url)
            .header(
                USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:129.0) Gecko/20100101 Firefox/129.0",
            )
            .send()
            .await
            .unwrap()
    }

    async fn get_bendrines(&mut self, dict: EkalbaDictionary, word: &str) -> Option<String> {
        let (dict_file, uuid_file) = match dict {
            EkalbaDictionary::Bendrines => (&mut self.bendrines_file, "bendrines_uuids"),
            EkalbaDictionary::Dabartines => (&mut self.dabartines_file, "dabartines_uuids"),
        };
        let file = if let Some(f) = &dict_file {
            f
        } else {
            let path = dirs::data_dir()
                .unwrap()
                .join("kalba")
                .join("language_data")
                .join(uuid_file);
            if !path.exists() {
                let contents = self.send_request(&format!("https://raw.githubusercontent.com/BrewingWeasel/kalba/main/data/{uuid_file}")).await.text_with_charset("utf-8").await.unwrap();
                fs::write(path.clone(), contents).unwrap();
            };
            let mut_dict_file = match dict {
                EkalbaDictionary::Bendrines => &mut self.bendrines_file,
                EkalbaDictionary::Dabartines => &mut self.dabartines_file,
            };
            *mut_dict_file = Some(fs::read_to_string(path).unwrap_or_default());
            mut_dict_file.as_ref().unwrap()
        };
        let dict_map = match dict {
            EkalbaDictionary::Bendrines => &mut self.ekalba_bendrines,
            EkalbaDictionary::Dabartines => &mut self.ekalba_dabartines,
        };
        dict_map
            .get_or_insert_with(|| {
                let mut words = HashMap::new();
                for i in file.lines() {
                    let (cur_word, uuid) = i.split_once('\t').unwrap();
                    words.insert(cur_word.to_owned(), uuid.to_owned());
                }
                words
            })
            .get(word)
            .cloned()
    }
}

fn get_def_from_file(
    lemma: &str,
    file: &str,
    dict_type: &DictFileType,
) -> Result<Definition, KalbaError> {
    match dict_type {
        DictFileType::StarDict => {
            let mut dict = stardict::no_cache(file)?;

            let mut def = String::new();
            if let Some(response) = stardict::StarDict::lookup(&mut dict, lemma)? {
                for word in &response {
                    if word.word != lemma {
                        continue;
                    }
                    for i in &word.segments {
                        def.push_str(&i.text);
                        def.push('\n');
                    }
                }
            }
            if def.is_empty() {
                Ok(Definition::Empty)
            } else {
                Ok(Definition::Text(def))
            }
        }
        DictFileType::TextSplitAt(delim) => Ok(fs::read_to_string(file).map(|lines| {
            for line in lines.lines() {
                let (word, def) = line.split_once(delim).unwrap();
                if word == lemma {
                    return Definition::Text(def.to_owned());
                }
            }
            Definition::Empty
        })?),
    }
}

async fn get_def_url(
    lemma: &str,
    url: &str,
    embed: bool,
    selector: &str,
) -> Result<Definition, KalbaError> {
    let new_url = url.replacen("{word}", lemma, 1);
    if embed {
        Ok(Definition::Text(format!(
            "<iframe class=\"w-full\" src=\"{}\"></iframe>",
            new_url
        )))
    } else {
        let client = reqwest::Client::new();
        let response = client.get(new_url).send().await?.text().await?;
        if selector.is_empty() {
            Ok(Definition::Text(response))
        } else {
            let mut text = String::new();
            let element_content_handlers = vec![text!(&selector, |t| {
                text.push_str(t.as_str());
                text.push('\n');
                Ok(())
            })];
            rewrite_str(
                &response,
                RewriteStrSettings {
                    element_content_handlers,
                    ..RewriteStrSettings::default()
                },
            )?;
            if text.is_empty() {
                Ok(Definition::Empty)
            } else {
                Ok(Definition::Text(text))
            }
        }
    }
}

async fn get_def_command(lemma: &str, cmd: &str) -> Result<Definition, KalbaError> {
    let real_command = cmd.replacen("{word}", lemma, 1);
    let output = run_command(&real_command)?;
    Ok(Definition::Text(String::from_utf8(output.stdout)?))
}

#[tauri::command]
pub async fn get_defs(
    state: State<'_, KalbaState>,
    lemma: String,
) -> Result<HashMap<String, Definition>, KalbaError> {
    let mut state = state.0.lock().await;
    let language = state
        .current_language
        .clone()
        .expect("current language should already be selected");
    if let Some(v) = state
        .language_cached_data
        .get(&language)
        .and_then(|v| v.definitions.get(&lemma))
    {
        Ok(v.clone())
    } else {
        let mut defs = HashMap::new();
        let mut successful_defs = Vec::new();
        for dict in &state
            .settings
            .languages
            .get(&language)
            .expect("language should exist")
            .dicts
        {
            if let Some(required_dictionary) = dict.run_when_not.as_ref() {
                if successful_defs.contains(&required_dictionary) {
                    continue;
                }
            }
            let def = if dict.fetch_by_default {
                get_def(
                    Arc::clone(&state.dict_info),
                    &dict.specific_settings,
                    &lemma,
                    &state.settings.definition_styling,
                )
                .await?
            } else {
                Definition::OnDemand(dict.name.to_owned())
            };
            if def != Definition::Empty {
                successful_defs.push(&dict.name);
            }

            defs.insert(dict.name.clone(), def);
        }
        state
            .language_cached_data
            .entry(language)
            .or_default()
            .definitions
            .insert(lemma, defs.clone());
        Ok(defs)
    }
}

#[tauri::command]
pub async fn get_definition_on_demand(
    state: State<'_, KalbaState>,
    lemma: String,
    dictionary: String,
) -> Result<Definition, KalbaError> {
    let state = state.0.lock().await;
    let language = state
        .current_language
        .clone()
        .expect("current language should already be selected");
    for dict in &state
        .settings
        .languages
        .get(&language)
        .expect("language should exist")
        .dicts
    {
        if dict.name == dictionary {
            return get_def(
                Arc::clone(&state.dict_info),
                &dict.specific_settings,
                &lemma,
                &state.settings.definition_styling,
            )
            .await;
        }
    }
    panic!("No dictionary found");
}

async fn get_def(
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
    dict: &DictionarySpecificSettings,
    lemma: &str,
    definition_styling: &DefinitionStyling,
) -> Result<Definition, KalbaError> {
    match dict {
        DictionarySpecificSettings::File(f, dict_type) => get_def_from_file(lemma, f, dict_type),
        DictionarySpecificSettings::Url(url, embed, selector) => {
            get_def_url(lemma, url, *embed, selector).await
        }
        DictionarySpecificSettings::Command(cmd) => get_def_command(lemma, cmd).await,
        DictionarySpecificSettings::EkalbaBendrines => {
            get_ekalba_bendrines(dict_info, lemma, definition_styling).await
        }
        DictionarySpecificSettings::EkalbaDabartines => {
            get_ekalba_dabartines(dict_info, lemma, definition_styling).await
        }
        DictionarySpecificSettings::Wiktionary(definition_lang, target_lang) => {
            get_wiktionary(
                dict_info,
                lemma,
                definition_lang,
                target_lang,
                definition_styling,
            )
            .await
        }
        DictionarySpecificSettings::WordReference(definition_lang, target_lang) => {
            get_wordreference(
                dict_info,
                lemma,
                definition_lang,
                target_lang,
                definition_styling,
            )
            .await
        }
    }
}

async fn get_wiktionary(
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
    lemma: &str,
    definition_lang: &str,
    target_lang: &str,
    definition_styling: &DefinitionStyling,
) -> Result<Definition, KalbaError> {
    let mut lock = dict_info.lock().await;
    let response = lock
        .send_request(&format!(
            "https://{definition_lang}.wiktionary.org/wiki/{lemma}"
        ))
        .await;
    let doc = Document::from_read(response.text().await?.as_bytes())?;
    let mut def_html = String::new();
    for node in doc.find(Attr("id", target_lang)) {
        let mut node = node.parent().unwrap();
        while let Some(cur_node) = node.next() {
            if cur_node.name() == Some("h2") || cur_node.children().any(|v| v.name() == Some("h2"))
            {
                break;
            }
            if cur_node.as_comment().is_none() && cur_node.attr("class") != Some("mw-editsection") {
                def_html.push_str(&cur_node.html());
            }
            node = cur_node;
        }
    }

    let element_content_handlers = vec![
        element!("ol li", |el| {
            el.set_attribute("style", &definition_styling.definition).unwrap();
            Ok(())
        }),
        element!(".h-usage-example", |el| {
            el.set_attribute("style", &definition_styling.info).unwrap();
            Ok(())
        }),
        element!(".headword-line", |el| {
            el.set_attribute("style", &definition_styling.main_detail).unwrap();
            Ok(())
        }),
        element!(".usage-label-sense, .antonym, .gender, ul li", |el| {
            el.set_attribute("style", &definition_styling.info).unwrap();
            Ok(())
        }),
        element!(".mw-heading4", |el| {
            el.set_attribute("style", "font-weight: bold; font-size: large;").unwrap();
            Ok(())
        }),
        // titles of sections we don't want
        element!("#Declension, #Declension_2, #Verb, #Verb_2, #Adjective, #Adjective_2, #Noun, #Noun_2, #Conjugation, #Conjugation_2, #Etymology, #Etymology_2, #References, #Further_reading", |el| {
            el.remove();
            Ok(())
        }),
        // wiktionary stuff we don't need
        element!(".mw-editsection, .catlinks, .NavFrame, .reference, .mw-references-wrap, .maintenance-line, .citation-whole, .sister-wikipedia, p", |el| {
            el.remove();
            Ok(())
        }),
    ];

    let finished = rewrite_str(
        &def_html,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )?;
    if finished.is_empty() {
        Ok(Definition::Empty)
    } else {
        Ok(Definition::Text(finished))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct DefinitionDetails {
    title: String,
    definition: String,
    notes: String,
    examples: String,
}

async fn get_wordreference(
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
    lemma: &str,
    definition_lang: &str,
    target_lang: &str,
    definition_styling: &DefinitionStyling,
) -> Result<Definition, KalbaError> {
    let mut lock = dict_info.lock().await;
    let response = lock
        .send_request(&format!(
            "https://www.wordreference.com/{target_lang}{definition_lang}/{lemma}"
        ))
        .await
        .text()
        .await?;

    let doc = Document::from_read(response.as_bytes()).unwrap();
    let Some(article_node) = doc
        .find(predicate::Descendant(
            predicate::Attr("id", "articleWRD"),
            predicate::Name("tbody"),
        ))
        .next()
    else {
        return Ok(Definition::Empty);
    };

    let children = article_node.children();

    let mut is_even = true;
    let mut definitions = vec![DefinitionDetails::default()];

    for row in children.skip(2) {
        if let Some(class) = row.attr("class") {
            let is_even_now = class == "even";
            if is_even_now != is_even {
                definitions.push(DefinitionDetails::default());
            }
            is_even = is_even_now;
        }
        let last_definition = definitions.last_mut().unwrap();
        for (i, cell) in row.children().enumerate() {
            match i {
                0 => {
                    for title_part in cell.children() {
                        if title_part.html().starts_with("<em") {
                            last_definition.title.push_str(&format!(
                                "<span style=\"{}\">{}</span>",
                                definition_styling.main_detail,
                                title_part.text()
                            ));
                        } else {
                            last_definition.title.push_str(&title_part.text());
                        }
                    }
                    last_definition.title.push('\n');
                }
                1 => {
                    let text = cell.text();
                    let final_text = text.trim();
                    let section = if final_text.starts_with('(') {
                        &mut last_definition.notes
                    } else {
                        &mut last_definition.examples
                    };
                    section.push_str(final_text);
                    section.push('\n');
                }
                2 => {
                    for definition_part in cell.children() {
                        if definition_part.html().starts_with("<em") {
                            last_definition.definition.push_str(&format!(
                                "<span style=\"{}\">{}</span>",
                                definition_styling.main_detail,
                                definition_part.text()
                            ));
                        } else {
                            last_definition.definition.push_str(&definition_part.text());
                        }
                    }
                    last_definition.definition.push('\n');
                }
                _ => continue,
            }
        }
    }
    let mut generated_html = String::new();
    let num_definitions = definitions.len();
    for (i, definition) in definitions.into_iter().enumerate() {
        generated_html.push_str(&format!(
            "<div><span style=\"{}\">{} </span>",
            definition_styling.info,
            definition.title.trim()
        ));
        if !definition.notes.is_empty() {
            generated_html.push_str(&format!(
                "<span style=\"{}\">{}</span></div>",
                definition_styling.main_detail,
                definition.notes.trim()
            ));
        }
        if !definition.definition.is_empty() {
            generated_html.push_str(&format!(
                "<p style=\"{}\">{}</p>",
                definition_styling.definition, definition.definition
            ));
        }
        if !definition.examples.is_empty() {
            generated_html.push_str(&format!("<p>{}</p>", definition.examples));
        }
        if i + 1 < num_definitions {
            generated_html.push_str("<hr>");
        }
    }
    Ok(Definition::Text(generated_html))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EkalbaRoot {
    pub details: EkalbaDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EkalbaDetails {
    pub view_html: String,
}

async fn get_ekalba_bendrines(
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
    word: &str,
    definition_styling: &DefinitionStyling,
) -> Result<Definition, KalbaError> {
    let response = {
        let mut lock = dict_info.lock().await;
        let Some(uuid) = lock.get_bendrines(EkalbaDictionary::Bendrines, word).await else {
            return Ok(Definition::Empty);
        };
        let uuid = uuid.to_owned();

        lock.send_request(&format!(
            "https://ekalba.lt/action/vocabulary/record/{uuid}?viewType=64"
        ))
        .await
        .json::<EkalbaRoot>()
        .await?
    };
    let element_content_handlers = vec![
        // Titles
        element!("span.bzpusjuodis", |el| {
            el.set_attribute("style", &definition_styling.main_detail)
                .unwrap();
            Ok(())
        }),
        element!("span.bzpaprastas", |el| {
            el.set_attribute("style", &definition_styling.definition)
                .unwrap();
            Ok(())
        }),
        element!("span.bzpetitas", |el| {
            el.set_attribute("style", &definition_styling.info).unwrap();
            Ok(())
        }),
        element!("p.bz-update-date", |el| {
            el.remove();
            Ok(())
        }),
    ];
    let response = rewrite_str(
        &response.details.view_html,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )?;

    if response.contains("Žodis įtrauktas į žodyno antraštyną. Informacija renkama.") {
        Ok(Definition::Empty)
    } else {
        Ok(Definition::Text(response))
    }
}

async fn get_ekalba_dabartines(
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
    word: &str,
    definition_styling: &DefinitionStyling,
) -> Result<Definition, KalbaError> {
    let response = {
        let mut lock = dict_info.lock().await;
        let Some(uuid) = lock.get_bendrines(EkalbaDictionary::Dabartines, word).await else {
            return Ok(Definition::Empty);
        };
        let uuid = uuid.to_owned();

        lock.send_request(&format!(
            "https://ekalba.lt/action/vocabulary/record/{uuid}?viewType=64"
        ))
        .await
        .json::<EkalbaRoot>()
        .await?
    };
    let element_content_handlers = vec![
        element!(".dz_homonym div", |el| {
            el.set_tag_name("span").unwrap();
            Ok(())
        }),
        element!(".dz_antraste", |el| {
            el.set_attribute("style", &definition_styling.main_detail)
                .unwrap();
            Ok(())
        }),
        element!(".dz_forms", |el| {
            el.set_attribute("style", &definition_styling.main_detail)
                .unwrap();
            Ok(())
        }),
        element!(".dz_valnumber", |el| {
            el.replace("<br>", ContentType::Html);
            Ok(())
        }),
        element!(".dz_tags", |el| {
            el.set_attribute("style", &definition_styling.info).unwrap();
            Ok(())
        }),
        text!("*", |t| {
            let content = t.as_str();
            if content.ends_with(':') && content.starts_with(' ') {
                t.replace(
                    &format!(
                        "<span style=\"{}\">{content}</span>",
                        &definition_styling.definition,
                    ),
                    ContentType::Html,
                )
            }
            Ok(())
        }),
    ];
    Ok(Definition::Text(rewrite_str(
        &response.details.view_html,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )?))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn get_delim_file() -> PathBuf {
        [
            env!("CARGO_MANIFEST_DIR"),
            "resources",
            "test",
            "sample_delim_dictionary",
        ]
        .iter()
        .collect()
    }

    fn get_stardict_file() -> PathBuf {
        [
            env!("CARGO_MANIFEST_DIR"),
            "resources",
            "test",
            "stardict",
            "Lithuanian-English Wiktionary dictionary.ifo",
        ]
        .iter()
        .collect()
    }

    #[test]
    fn read_from_delimiter_file_key_exists() {
        let f = get_delim_file();
        let dict_type = DictFileType::TextSplitAt(String::from(":"));
        assert_eq!(
            get_def_from_file("geras", f.to_str().unwrap(), &dict_type).unwrap(),
            Definition::Text(String::from("good"))
        );
    }

    #[test]
    fn read_from_delimiter_file_key_doesnt_exist() {
        let f = get_delim_file();
        let dict_type = DictFileType::TextSplitAt(String::from(":"));
        assert_eq!(
            get_def_from_file("", f.to_str().unwrap(), &dict_type).unwrap(),
            Definition::Empty
        );
    }

    #[test]
    fn read_from_delimiter_file_definition_has_delim() {
        let f = get_delim_file();
        let dict_type = DictFileType::TextSplitAt(String::from(":"));
        assert_eq!(
            get_def_from_file("blogas", f.to_str().unwrap(), &dict_type).unwrap(),
            Definition::Text(String::from("bad:extra"))
        );
    }

    #[test]
    fn read_from_delimiter_file_file_doesnt_exist() {
        let dict_type = DictFileType::TextSplitAt(String::from(":"));
        assert_eq!(
            get_def_from_file("blogas", "ee", &dict_type)
                .unwrap_err()
                .to_string(),
            String::from("No such file or directory (os error 2)")
        );
    }

    #[test]
    fn read_from_stardict_key_exists() {
        let f = get_stardict_file();
        let dict_type = DictFileType::StarDict;
        assert_eq!(
            get_def_from_file("blogas", f.to_str().unwrap(), &dict_type).unwrap(),
            Definition::Text(String::from("<i>adj</i><br><ol><li>bad, wrong</li></ol><br><i>noun</i><br><ol><li>(Internet) blog</li></ol>\n"))
        );
    }

    #[test]
    fn read_from_stardict_key_doesnt_exist() {
        let f = get_stardict_file();
        let dict_type = DictFileType::StarDict;
        assert_eq!(
            get_def_from_file("this key doesnt exist", f.to_str().unwrap(), &dict_type).unwrap(),
            Definition::Empty,
        );
    }

    #[test]
    fn read_from_stardict_file_doesnt_exist() {
        let dict_type = DictFileType::StarDict;
        assert_eq!(
            get_def_from_file("", "this file doesnt exist", &dict_type)
                .unwrap_err()
                .to_string(),
            String::from("Dict path is not invalid") // TODO: actual error message is wrong
        );
    }
}
