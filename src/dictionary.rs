use std::fs;

// TODO: allow getting from server
pub fn get_def(lemma: &str) -> String {
    if let Ok(lines) = fs::read_to_string("dicts/lithuanian") {
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
