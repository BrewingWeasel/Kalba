use std::fs;

use crate::settings::Dictionary;

fn get_def(lemma: &str, file: &str) -> String {
    if let Ok(lines) = fs::read_to_string(file) {
        for line in lines.lines() {
            let (word, def) = line.split_once(":\t").unwrap();
            if word == lemma {
                return def.to_owned();
            }
        }
        String::from("unknown")
    } else {
        String::from("failed to load dictionary")
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
            Dictionary::File(f) => get_def(lemma, f),
            Dictionary::Url(url) => get_def_url(lemma, url),
        };
        defs.push(def);
    }
    defs
}
