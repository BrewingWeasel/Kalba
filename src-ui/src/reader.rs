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

async fn export_card(sent: &str, word: &str, defs: &Vec<String>, settings: &Settings) {
    #[allow(clippy::single_match)]
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
            morph: HashMap::new(),
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
                let def: SakinyjeResult<String> =
                    tauri::invoke("get_def", &GetDefEvent { lemma, dict })
                        .await
                        .unwrap();
                // defs.push(def)
                let def: Result<String, String> = def.into();
                defs.push(def.unwrap_or_else(|e| format!("Error: {e}")));
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
pub fn ReaderView(settings: Resource<(), Settings>) -> impl IntoView {
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
        <div class="input">
            <form on:submit=on_submit>
                <input type="textarea" class="sentsubmit" value=sentence node_ref=input_element/>
                <input class="parsebutton" type="submit" value="Parse"/>
            </form>
        </div>
        <div class="sentence">
             {move || conts.get().map_or_else(|| view! { <p>"Loading..."</p> }.into_view(), |data| data.into_iter()
                         .enumerate()
                         .map(|(i, d)| view! { <Word word=d i=i word_selector=set_selected_word/> })
                         .collect::<Vec<_>>()
                         .into_view())}
        </div>
        <br/>
        <div class="wordinfo">
            {move || {
                if selected_word().is_some() {
                    view! {
                        <input
                            class="selectedword"
                            type="text"
                            on:change=move |ev| {
                                conts
                                    .update(|v| {
                                        v
                                            .as_mut()
                                            .unwrap()
                                            .get_mut(selected_word().unwrap())
                                            .unwrap()
                                            .lemma = event_target_value(&ev);
                                    });
                                definition.refetch();
                            }

                            prop:value=move || {
                                selected_word
                                    .get()
                                    .and_then(|i| {
                                        let words = conts.get().unwrap();
                                        words.get(i).cloned()
                                    })
                                    .map(|v| v.lemma)
                            }
                        />
                    }
                        .into_view()
                } else {
                    view! {}.into_view()
                }
            }}

        </div>

        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {definition
                .get()
                .map(|data| {
                    data.iter()
                        .map(|d| {
                            view! {
                                <div class="definition" inner_html=d></div>
                                <br/>
                            }
                        })
                        .collect_view()
                })}
            {move || {
                selected_word()
                    .map(|i| {
                        view! {
                            <div class="export">
                                <button
                                    on:click=move |_| {
                                        let cur_conts = conts().unwrap();
                                        let lemma = cur_conts[i].lemma.clone();
                                        console_log(&lemma);
                                        spawn_local(async move {
                                            export_card(
                                                    &sentence(),
                                                    &lemma,
                                                    defs().get(&lemma).unwrap(),
                                                    &settings().unwrap(),
                                                )
                                                .await;
                                        });
                                        console_log(&format!("{:#?}", &defs()));
                                    }

                                    class="exportbutton"
                                >
                                    export to anki
                                </button>
                            </div>
                        }
                    })
            }}

        </Suspense>
    }
}

#[component]
fn Word(word: Word, i: usize, word_selector: WriteSignal<Option<usize>>) -> impl IntoView {
    let mut class = String::from("word");
    if !word.clickable {
        class.push_str(" punctuation ");
    }
    for (feat, value) in &word.morph {
        class.push_str(&format!(" {feat}-{value}"));
    }
    view! {
        <span
            class=class
            on:click=move |_| {
                if word.clickable {
                    word_selector.set(Some(i));
                }
            }
        >

            {&word.text}
        </span>
    }
}
