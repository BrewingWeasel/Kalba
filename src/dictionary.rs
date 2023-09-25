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

// TODO: allow getting from server
pub fn get_defs(lemma: &str, dicts: &Vec<Dictionary>) -> Vec<String> {
    let mut defs = Vec::new();
    for dict in dicts {
        let def = match dict {
            Dictionary::File(f) => get_def(lemma, f),
            _ => unimplemented!(),
        };
        defs.push(def);
    }
    defs
}
