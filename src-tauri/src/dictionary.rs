use shared::*;
use std::{error::Error, fs};

fn get_def_from_file(
    lemma: &str,
    file: &str,
    dict_type: &DictFileType,
) -> Result<String, Box<dyn Error>> {
    match dict_type {
        DictFileType::StarDict => {
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
pub async fn get_def(dict: Dictionary, lemma: &str) -> Result<SakinyjeResult<String>, String> {
    match dict {
        Dictionary::File(f, dict_type) => Ok(get_def_from_file(lemma, &f, &dict_type).into()),
        Dictionary::Url(url) => Ok(get_def_url(lemma, &url).await.into()),
    }
}
