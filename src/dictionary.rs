use scraper::{Html, Selector};
use serde::Deserialize;
use serde_derive::Serialize;
use std::fs;

use crate::settings::Dictionary;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum DictFileType {
    TextSplitAt(String),
    StarDict,
}

fn get_def(lemma: &str, file: &str, dict_type: &DictFileType) -> String {
    match dict_type {
        DictFileType::StarDict => {
            let mut dict = stardict::no_cache(file).unwrap();

            let mut def = String::new();
            if let Ok(response) = stardict::StarDict::lookup(&mut dict, lemma) {
                for word in &response.unwrap() {
                    if word.word != lemma {
                        continue;
                    }
                    for i in &word.segments {
                        let fragment = Html::parse_fragment(&i.text);
                        let selector = Selector::parse("li").unwrap();

                        for element in fragment.select(&selector) {
                            assert_eq!("li", element.value().name());
                            def.push_str(&element.text().fold(String::new(), |acc, n| acc + n));
                            def.push('\n')
                        }
                    }
                }
            }
            def
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

fn get_def_url(lemma: &str, url: &str) -> String {
    println!("getting definition from {url}");
    let new_url = url.replacen("{word}", lemma, 1);
    let client = reqwest::blocking::Client::new();
    client.get(new_url).send().unwrap().text().unwrap() // TODO: async
}

// TODO: allow getting from server
pub fn get_defs(lemma: &str, dicts: &Vec<Dictionary>) -> Vec<String> {
    let mut defs = Vec::new();
    for dict in dicts {
        let def = match dict {
            Dictionary::File(f, dict_type) => get_def(lemma, f, dict_type),
            Dictionary::Url(url) => get_def_url(lemma, url),
        };
        defs.push(def);
    }
    defs
}
