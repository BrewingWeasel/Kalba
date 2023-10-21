use std::collections::HashMap;

use leptos::{
    html::Input,
    leptos_dom::logging::{console_error, console_log},
    *,
};
use serde::Serialize;
use shared::*;
use tauri_sys::tauri;
use web_sys::SubmitEvent;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[derive(Serialize)]
struct GetDefEvent<'a> {
    dict: &'a Dictionary,
    lemma: &'a str,
}

#[derive(Serialize)]
pub struct ParsingInfo<'a> {
    pub sent: &'a str,
    pub model: &'a str,
}

#[derive(Serialize)]
pub struct AddToAnki<'a> {
    pub sent: &'a str,
    pub word: &'a str,
    pub defs: &'a Vec<String>,
    pub settings: &'a Settings,
}

async fn get_settings() -> Settings {
    tauri::invoke("get_settings", &()).await.unwrap()
}

async fn export_card(sent: &str, word: &str, defs: &Vec<String>, settings: &Settings) {
    match tauri::invoke(
        "add_to_anki",
        &AddToAnki {
            sent,
            word,
            defs,
            settings,
        },
    )
    .await
    {
        Err(e) => console_error(&e.to_string()),
        Ok(()) => (),
    }
}

async fn send_sentence(sent: String) -> Vec<Word> {
    match tauri::invoke(
        "parse_text",
        &ParsingInfo {
            sent: &sent,
            model: "lt_core_news_sm",
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

async fn get_definition<'a>(
    lemma_id: Option<usize>,
    words: Resource<String, Vec<Word>>,
    defs: ReadSignal<HashMap<String, Vec<String>>>,
    writable_defs: WriteSignal<HashMap<String, Vec<String>>>,
    settings: Resource<(), Settings>,
) -> Vec<String> {
    console_log(&format!("{:#?}", words.get()));
    // TODO: lots of clones
    if let Some(i) = lemma_id {
        let lemma = &words().unwrap()[i].lemma;
        let defs = move || defs.get();
        if let Some(cached_defs) = defs().get(lemma) {
            cached_defs.to_owned()
        } else {
            let mut defs = Vec::new();
            for dict in &settings.get().unwrap().dicts {
                let def: String = tauri::invoke("get_def", &GetDefEvent { lemma, dict })
                    .await
                    .unwrap();
                defs.push(def);
            }
            writable_defs.update(|v| {
                v.insert(lemma.clone(), defs.clone());
            });
            defs
        }
    } else {
        Vec::new()
    }
}

#[component]
fn App() -> impl IntoView {
    let settings = create_resource(|| (), |_| async move { get_settings().await });

    let (defs, change_defs) = create_signal(HashMap::new());

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
        move |v| get_definition(v, conts, defs, change_defs, settings),
    );

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
        { move || if selected_word().is_some() {
            view! {<input type="text" on:change=move |ev| {
                conts.update(|v| {
                    v.as_mut().unwrap().get_mut(selected_word().unwrap()).unwrap().lemma = event_target_value(&ev);
                });
                definition.refetch();
            } prop:value={move || {
            selected_word
                .get()
                .and_then(|i| {
                    let words = conts.get().unwrap();
                    words.get(i).cloned()
                })
                .map(|v| v.lemma)
        }}></input>}.into_view()} else {
        view! {}.into_view()
        }
        }
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {definition.get().map(|data| data.iter().map(|d| view! { <p>{d}</p> }).collect_view())}
            {move || selected_word().map(|i| view !{
             <button
                on:click=move |_| {
                let cur_conts = conts().unwrap();
                let lemma = cur_conts[i].lemma.clone();
                console_log(&lemma);
                spawn_local(async move {
                        export_card(&sentence(), &lemma, defs().get(&lemma).unwrap(), &settings().unwrap()).await;
                });
                console_log(&format!("{:#?}", &defs()));
            }>export to anki</button>})}
        </Suspense>
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
