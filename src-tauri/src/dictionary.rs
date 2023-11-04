use shared::*;
use std::{error::Error, fs};
use tauri::State;

use crate::SakinyjeState;

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
    println!("getting definition from {new_url}");
    let client = reqwest::Client::new();
    Ok(client.get(new_url).send().await?.text().await?)
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
        for dict in &state.settings.dicts {
            let def = get_def(dict, &lemma).await.into();
            defs.push(def);
        }
        state.to_save.cached_defs.insert(lemma, defs.clone());
        Ok(defs)
    }
}

async fn get_def(dict: &Dictionary, lemma: &str) -> Result<String, Box<dyn Error>> {
    match dict {
        Dictionary::File(f, dict_type) => get_def_from_file(lemma, &f, &dict_type),
        Dictionary::Url(url) => get_def_url(lemma, &url).await,
    }
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
