use lol_html::{element, html_content::ContentType, rewrite_str, text, RewriteStrSettings};
use reqwest::Client;
use select::{document::Document, predicate::Attr};
use serde::{Deserialize, Serialize};
use shared::*;
use std::{collections::HashMap, error::Error, fs, sync::Arc};
use tauri::State;

use crate::{commands::run_command, SakinyjeState};

// TODO: should be customizable
const DEFINITION: &str = "color: #eb6f92; font-weight: 900;";
const MAIN_DETAIL: &str = "color: #f6c177; font-weight: bold; font-size: large;";
const INFO: &str = "color: #c4a7e7; font-style: italic;";

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
            .get_or_insert_with(|| Client::new())
            .get(url)
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
                .join("sakinyje")
                .join("language_data")
                .join(uuid_file);
            if !path.exists() {
                let contents = self.send_request(&format!("https://raw.githubusercontent.com/BrewingWeasel/sakinyje/main/data/{uuid_file}")).await.text_with_charset("utf-8").await.unwrap();
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
) -> Result<String, Box<dyn Error>> {
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
            Ok(def)
        }
        DictFileType::TextSplitAt(delim) => Ok(fs::read_to_string(file).map(|lines| {
            for line in lines.lines() {
                let (word, def) = line.split_once(delim).unwrap();
                if word == lemma {
                    return def.to_owned();
                }
            }
            String::new()
        })?),
    }
}

async fn get_def_url(lemma: &str, url: &str) -> Result<String, Box<dyn Error>> {
    let new_url = url.replacen("{word}", lemma, 1);
    let client = reqwest::Client::new();
    Ok(client.get(new_url).send().await?.text().await?)
}

async fn get_def_command(lemma: &str, cmd: &str) -> Result<String, Box<dyn Error>> {
    let real_command = cmd.replacen("{word}", lemma, 1);
    let output = run_command(&real_command)?;
    Ok(String::from_utf8(output.stdout)?)
}

#[tauri::command]
pub async fn get_defs(
    state: State<'_, SakinyjeState>,
    lemma: String,
) -> Result<Vec<SakinyjeResult<String>>, String> {
    let mut state = state.0.lock().await;
    if let Some(v) = state.to_save.cached_defs.get(&lemma) {
        Ok(v.clone())
    } else {
        let mut defs = Vec::new();
        for (_, dict) in &state.settings.dicts {
            let def = get_def(Arc::clone(&state.dict_info), dict, &lemma)
                .await
                .into();
            defs.push(def);
        }
        state.to_save.cached_defs.insert(lemma, defs.clone());
        Ok(defs)
    }
}

async fn get_def(
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
    dict: &Dictionary,
    lemma: &str,
) -> Result<String, Box<dyn Error>> {
    match dict {
        Dictionary::File(f, dict_type) => get_def_from_file(lemma, f, dict_type),
        Dictionary::Url(url) => get_def_url(lemma, url).await,
        Dictionary::Command(cmd) => get_def_command(lemma, cmd).await,
        Dictionary::EkalbaBendrines => Ok(get_ekalba_bendrines(dict_info, lemma).await),
        Dictionary::EkalbaDabartines => Ok(get_ekalba_dabartines(dict_info, lemma).await),
        Dictionary::Wiktionary(definition_lang, target_lang) => {
            Ok(get_wiktionary(dict_info, lemma, definition_lang, target_lang).await)
        }
    }
}

async fn get_wiktionary(
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
    lemma: &str,
    definition_lang: &str,
    target_lang: &str,
) -> String {
    let mut lock = dict_info.lock().await;
    let response = lock
        .send_request(&format!(
            "https://{definition_lang}.wiktionary.org/wiki/{lemma}"
        ))
        .await;
    let doc = Document::from_read(response.text().await.unwrap().as_bytes()).unwrap();
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
            el.set_attribute("style", DEFINITION).unwrap();
            Ok(())
        }),
        element!(".h-usage-example", |el| {
            el.set_attribute("style", INFO).unwrap();
            Ok(())
        }),
        element!(".headword-line", |el| {
            el.set_attribute("style", MAIN_DETAIL).unwrap();
            Ok(())
        }),
        element!(".usage-label-sense, .antonym, .gender, ul li", |el| {
            el.set_attribute("style", INFO).unwrap();
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

    rewrite_str(
        &def_html,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .unwrap()
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
) -> String {
    let response = {
        let mut lock = dict_info.lock().await;
        let Some(uuid) = lock.get_bendrines(EkalbaDictionary::Bendrines, word).await else {
            return String::new();
        };
        let uuid = uuid.to_owned();

        lock.send_request(&format!(
            "https://ekalba.lt/action/vocabulary/record/{uuid}?viewType=64"
        ))
        .await
        .json::<EkalbaRoot>()
        .await
        .unwrap()
    };
    let element_content_handlers = vec![
        // Titles
        element!("span.bzpusjuodis", |el| {
            el.set_attribute("style", MAIN_DETAIL).unwrap();
            Ok(())
        }),
        element!("span.bzpaprastas", |el| {
            el.set_attribute("style", DEFINITION).unwrap();
            Ok(())
        }),
        element!("span.bzpetitas", |el| {
            el.set_attribute("style", INFO).unwrap();
            Ok(())
        }),
        element!("p.bz-update-date", |el| {
            el.remove();
            Ok(())
        }),
    ];
    rewrite_str(
        &response.details.view_html,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .unwrap()
}

async fn get_ekalba_dabartines(
    dict_info: Arc<tauri::async_runtime::Mutex<DictionaryInfo>>,
    word: &str,
) -> String {
    let response = {
        let mut lock = dict_info.lock().await;
        let Some(uuid) = lock.get_bendrines(EkalbaDictionary::Dabartines, word).await else {
            return String::new();
        };
        let uuid = uuid.to_owned();

        lock.send_request(&format!(
            "https://ekalba.lt/action/vocabulary/record/{uuid}?viewType=64"
        ))
        .await
        .json::<EkalbaRoot>()
        .await
        .unwrap()
    };
    let element_content_handlers = vec![
        element!(".dz_homonym div", |el| {
            el.set_tag_name("span").unwrap();
            Ok(())
        }),
        element!(".dz_antraste", |el| {
            el.set_attribute("style", MAIN_DETAIL).unwrap();
            Ok(())
        }),
        element!(".dz_forms", |el| {
            el.set_attribute("style", MAIN_DETAIL).unwrap();
            Ok(())
        }),
        element!(".dz_valnumber", |el| {
            el.replace("<br>", ContentType::Html);
            Ok(())
        }),
        element!(".dz_tags", |el| {
            el.set_attribute("style", INFO).unwrap();
            Ok(())
        }),
        text!("*", |t| {
            let content = t.as_str();
            if content.ends_with(':') && content.starts_with(' ') {
                t.replace(
                    &format!("<span style=\"{DEFINITION}\">{content}</span>"),
                    ContentType::Html,
                )
            }
            Ok(())
        }),
    ];
    rewrite_str(
        &response.details.view_html,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .unwrap()
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
            String::from("good")
        );
    }

    #[test]
    fn read_from_delimiter_file_key_doesnt_exist() {
        let f = get_delim_file();
        let dict_type = DictFileType::TextSplitAt(String::from(":"));
        assert_eq!(
            get_def_from_file("", f.to_str().unwrap(), &dict_type).unwrap(),
            String::new()
        );
    }

    #[test]
    fn read_from_delimiter_file_definition_has_delim() {
        let f = get_delim_file();
        let dict_type = DictFileType::TextSplitAt(String::from(":"));
        assert_eq!(
            get_def_from_file("blogas", f.to_str().unwrap(), &dict_type).unwrap(),
            String::from("bad:extra")
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
            String::from("<i>adj</i><br><ol><li>bad, wrong</li></ol><br><i>noun</i><br><ol><li>(Internet) blog</li></ol>\n")
        );
    }

    #[test]
    fn read_from_stardict_key_doesnt_exist() {
        let f = get_stardict_file();
        let dict_type = DictFileType::StarDict;
        assert_eq!(
            get_def_from_file("this key doesnt exist", f.to_str().unwrap(), &dict_type).unwrap(),
            String::new(),
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
