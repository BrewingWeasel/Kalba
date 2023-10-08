use eframe::egui::Context;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_derive::Serialize;
use std::{error::Error, fs, sync::mpsc::Sender};

use crate::settings::Dictionary;

#[derive(Serialize, Deserialize, PartialEq, Clone, Eq)]
pub enum DictFileType {
    TextSplitAt(String),
    StarDict,
}

fn get_def(lemma: &str, file: &str, dict_type: &DictFileType) -> String {
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
                            let fragment = Html::parse_fragment(&i.text);
                            let selector = Selector::parse("li")?;

                            for element in fragment.select(&selector) {
                                assert_eq!("li", element.value().name());
                                def.push_str(&element.text().fold(String::new(), |acc, n| acc + n));
                                def.push('\n')
                            }
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

// TODO: REFERENCES NOT CLONES AHHH
pub fn get_defs(lemma: String, dicts: Vec<Dictionary>, tx: Sender<Vec<String>>, ctx: Context) {
    tokio::spawn(async move {
        let mut defs = Vec::new();
        for dict in &dicts {
            let def = match dict {
                Dictionary::File(f, dict_type) => get_def(&lemma, f, dict_type),
                Dictionary::Url(url) => get_def_url(&lemma, url).await,
            };
            defs.push(def);
        }
        let _ = tx.send(defs);
        ctx.request_repaint();
    });
}
