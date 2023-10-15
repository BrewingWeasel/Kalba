// use eframe::egui::Context;
// use futures::future::join_all;
// use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

// use crate::settings::Dictionary;

#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Debug)]
pub enum DictFileType {
    TextSplitAt(String),
    StarDict,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Dictionary {
    File(String, DictFileType), // Eventually these will have specific things
    Url(String),
}

fn get_def_from_file(lemma: &str, file: &str, dict_type: &DictFileType) -> String {
    match dict_type {
        DictFileType::StarDict => {
            let get_data = || -> Result<String, Box<dyn Error>> {
                let mut dict = stardict::no_cache(file)?;

                let mut def = String::new();
                if let Ok(response) = stardict::StarDict::lookup(&mut dict, lemma) {
                    for word in &response.ok_or("")? {
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
            };
            get_data().unwrap_or(String::from("unknown"))
        }
        DictFileType::TextSplitAt(delim) => {
            if let Ok(lines) = fs::read_to_string(file) {
                for line in lines.lines() {
                    let (word, def) = line.split_once(delim).unwrap();
                    if word == lemma {
                        return def.to_owned();
                    }
                }
                String::from("unknown")
            } else {
                String::from("failed to load dictionary")
            }
        }
    }
}

async fn get_def_url(lemma: &str, url: &str) -> String {
    println!("getting definition from {url}");
    let new_url = url.replacen("{word}", lemma, 1);
    let client = reqwest::Client::new();
    client
        .get(new_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

#[tauri::command]
pub async fn get_def(dict: Dictionary, lemma: &str) -> Result<String, String> {
    match dict {
        Dictionary::File(f, dict_type) => Ok(get_def_from_file(lemma, &f, &dict_type)),
        Dictionary::Url(url) => Ok(get_def_url(lemma, &url).await),
    }
}

// pub fn get_defs(lemma: Arc<str>, dicts: Vec<Dictionary>, tx: Sender<Vec<String>>, ctx: Context) {
//     tokio::spawn(async move {
//         let defs = join_all(dicts.iter().map(|d| get_def(d, &lemma))).await;
//         let _ = tx.send(defs);
//         ctx.request_repaint();
//     });
// }
