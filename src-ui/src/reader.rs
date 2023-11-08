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
}

async fn export_card(sent: &str, word: &str, defs: &Vec<String>) {
    #[allow(clippy::single_match)]
    match tauri::invoke("add_to_anki", &AddToAnki { sent, word, defs }).await {
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
            rating: 0,
            morph: HashMap::new(),
            clickable: false,
        }],
    }
}

async fn get_definition<'a>(
    lemma_id: Option<usize>,
    words: Resource<String, Vec<Word>>,
) -> Vec<SakinyjeResult<String>> {
    console_log(&format!("{:#?}", words.get()));
    if let Some(i) = lemma_id {
        let lemma = &words().unwrap()[i].lemma;
        tauri::invoke("get_defs", &GetDefEvent { lemma })
            .await
            .unwrap()
    } else {
        Vec::new()
    }
}

#[component]
pub fn ReaderView() -> impl IntoView {
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

    view! {
        <div class="input">
            <form on:submit=on_submit>
                <input type="textarea" class="sentsubmit" value=sentence node_ref=input_element/>
                <input class="parsebutton" type="submit" value="Parse"/>
            </form>
        </div>
        <div class="sentence">
            {move || {
                conts
                    .get()
                    .map_or_else(
                        || view! { <p>"Loading..."</p> }.into_view(),
                        |data| {
                            data.into_iter()
                                .enumerate()
                                .map(|(i, d)| {
                                    view! { <Word word=d i=i word_selector=set_selected_word/> }
                                })
                                .collect::<Vec<_>>()
                                .into_view()
                        },
                    )
            }}

        </div>
        <br/>
        {move || {
            if selected_word().is_some() {
                view! {
                    <div class="wordinfo">
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

                    </div>

                    <div class="grammarinfo">
                        {move || {
                            conts
                                .get()
                                .unwrap()[selected_word().unwrap()]
                                .morph
                                .iter()
                                .map(|(k, v)| {
                                    view! {
                                        <div class="grammarfeature">
                                            <span class=k>{k}</span>
                                            <span class="seperator">:</span>
                                            <span class=v>{v}</span>
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}

                    </div>
                    <hr/>
                }
                    .into_view()
            } else {
                view! {}.into_view()
            }
        }}

        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {definition
                .get()
                .map(|data| {
                    data.iter()
                        .map(|d| {
                            match d {
                                SakinyjeResult::Ok(s) => {
                                    view! {
                                        <div class="definition" inner_html=s></div>
                                        <br/>
                                    }
                                        .into_view()
                                }
                                SakinyjeResult::Err(v) => {
                                    view! {
                                        <div class="error">Err: {v}</div>
                                        <br/>
                                    }
                                        .into_view()
                                }
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
                                                    &definition()
                                                        .unwrap()
                                                        .into_iter()
                                                        .filter_map(|d| {
                                                            Into::<Result<String, String>>::into(d).ok()
                                                        })
                                                        .collect(),
                                                )
                                                .await;
                                        });
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
        class.push_str(" punctuation");
    }
    for (feat, value) in &word.morph {
        class.push_str(&format!(" {feat}-{value}"));
    }
    class.push_str(" rating-");
    class.push_str(&word.rating.to_string());
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
