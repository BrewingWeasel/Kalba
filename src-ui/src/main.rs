use leptos::{html::Input, *};
use serde::Serialize;
use tauri_sys::tauri;
use web_sys::SubmitEvent;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[derive(Serialize)]
struct GetDefEvent<'a> {
    dict: Dictionary,
    lemma: &'a str,
}

#[derive(Serialize, PartialEq, Clone, Eq, Debug)]
pub enum DictFileType {
    TextSplitAt(String),
    StarDict,
}

#[derive(Serialize, PartialEq, Eq, Clone, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Dictionary {
    File(String, DictFileType), // Eventually these will have specific things
    Url(String),
}

#[derive(Serialize)]
struct ParsingInfo<'a> {
    sent: &'a str,
    model: &'a str,
}

#[derive(Serialize)]
struct SentenceEvent<'a> {
    sent: ParsingInfo<'a>,
}

// TODO: shared crate
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Word {
    pub text: String,
    pub lemma: String,
    pub morph: Option<String>,
    pub clickable: bool,
}

async fn send_sentence(sent: String) -> Vec<Word> {
    match tauri::invoke(
        "parse_text",
        &SentenceEvent {
            sent: ParsingInfo {
                sent: &sent,
                model: "lt_core_news_sm",
            },
        },
    )
    .await
    {
        Ok(v) => v,
        Err(e) => vec![Word {
            text: e.to_string(),
            lemma: e.to_string(),
            morph: None,
            clickable: false,
        }],
    }
}

async fn get_definition(lemma_id: Option<usize>, words: Resource<String, Vec<Word>>) -> String {
    if let Some(i) = lemma_id {
        tauri::invoke(
            "get_def",
            &GetDefEvent {
                lemma: &words.get().unwrap()[i].lemma,
                dict: Dictionary::File(
                    "/home/weasel/rust/language/dicts/lithuanian".to_string(),
                    DictFileType::TextSplitAt(":\t".to_string()),
                ),
            },
        )
        .await
        .unwrap()
    } else {
        String::new()
    }
}

#[component]
fn App() -> impl IntoView {
    let (sentence, set_sentence) = create_signal(String::new());

    let (selected_word, set_selected_word) = create_signal(None);

    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        set_sentence(value);
    };
    let conts = create_local_resource(move || sentence.get(), send_sentence);
    let definition = create_resource(
        move || selected_word.get(),
        move |v| get_definition(v, conts),
    );

    let selected_lemma = move || {
        selected_word
            .get()
            .and_then(|i| {
                let words = conts.get().unwrap();
                words.get(i).cloned()
            })
            .map(|v| v.lemma)
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=sentence
                node_ref=input_element
            />
            <input type="submit" value="Parse"/>
        </form>
        <div class="sentence">{move || match conts.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => data.into_iter().enumerate().map(|(i, d)| view! { <Word word={d} i=i word_selector=set_selected_word /> }).collect::<Vec<_>>().into_view(),
        }}</div>
        <div class="selectedword">{selected_lemma}</div>
        <div class="definition">{move || match definition.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => view! { <p>{data}</p> }.into_view(),
        }}</div>
    }
}

#[component]
fn Word(word: Word, i: usize, word_selector: WriteSignal<Option<usize>>) -> impl IntoView {
    let mut class = String::from("word");
    if !word.clickable {
        class.push_str(" punctuation");
    }
    if let Some(morph) = word.morph {
        class.push(' ');
        class.push_str(&morph);
    }
    view! { <span class=class on:click=move |_| {
        word_selector.set(Some(i));
    }>{&word.text}</span> }
}
