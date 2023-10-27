use std::collections::HashMap;

use leptos::{
    html::Input,
    leptos_dom::logging::{console_error, console_log},
    *,
};
use leptos_router::*;
use serde::Serialize;
use shared::*;
use tauri_sys::{dialog::FileDialogBuilder, tauri};
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

fn get_folder(writer: WriteSignal<String>) {
    wasm_bindgen_futures::spawn_local(async move {
        console_log("getting file");
        if let Ok(Some(v)) = FileDialogBuilder::new().pick_folder().await {
            console_log(&format!("{:?}", v));
            writer(v.to_string_lossy().to_string());
        }
    })
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
    view! {
        <Router>
            <nav>
                <NavBar/>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=move || ReaderView(ReaderViewProps { settings })/>
                    <Route path="/reader" view=move || ReaderView(ReaderViewProps { settings })/>
                    <Route
                        path="/settings"
                        view=move || SettingsChanger(SettingsChangerProps {
                            settings: settings,
                        })
                    />
                    <Route path="/dictionary" view=Todo/>
                    <Route path="/corpus" view=Todo/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <div class="navbar">
            <A href="settings">Settings</A>
            <A href="/reader">Reader</A>
            <A href="dictionary">Dictionary</A>
        </div>
    }
}

#[component]
fn Todo() -> impl IntoView {
    view! { <p>Coming soon!</p> }
}

#[component]
fn SimpleTextSetting(
    readsig: ReadSignal<String>,
    writesig: WriteSignal<String>,
    name: &'static str,
    desc: &'static str,
) -> impl IntoView {
    view! {
        <label for=name>{desc}</label>
        <input
            id=name
            type="text"
            on:input=move |ev| {
                writesig(event_target_value(&ev));
            }

            prop:value=readsig
        />
    }
}

#[component]
fn SettingsChanger(settings: Resource<(), Settings>) -> impl IntoView {
    let old_settings = if let Some(s) = settings() {
        s
    } else {
        return view! { <p>Unable to load settings</p> }.into_view();
    };
    let (model, set_model) = create_signal(old_settings.model);
    let (deck, set_deck) = create_signal(old_settings.deck);
    let (note, set_note) = create_signal(old_settings.note_type);
    view! {
        <SimpleTextSetting readsig=model writesig=set_model name="model" desc="SpaCy Model"/>
        <button on:click=move |_| {
            console_log("pirmas");
            get_folder(set_model);
        }>open</button>
        <hr/>
        <SimpleTextSetting readsig=deck writesig=set_deck name="deck" desc="Anki Deck"/>
        <SimpleTextSetting readsig=note writesig=set_note name="note" desc="Note type"/>

        <hr/>
        <DictionaryList initial_dicts=old_settings.dicts/>

        <hr/>

        <button on:click=move |_| {
            settings
                .update(|v| {
                    v.as_mut().unwrap().model = model();
                });
        }>save</button>
    }
    .into_view()
}

#[component]
fn ReaderView(settings: Resource<(), Settings>) -> impl IntoView {
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
            {move || match conts.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(data) => {
                    data.into_iter()
                        .enumerate()
                        .map(|(i, d)| view! { <Word word=d i=i word_selector=set_selected_word/> })
                        .collect::<Vec<_>>()
                        .into_view()
                }
            }}

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
        class.push_str(" punctuation");
    }
    if let Some(morph) = word.morph {
        class.push(' ');
        class.push_str(&morph);
    }
    view! {
        <span
            class=class
            on:click=move |_| {
                word_selector.set(Some(i));
            }
        >

            {&word.text}
        </span>
    }
}

#[component]
fn DictionaryList(initial_dicts: Vec<Dictionary>) -> impl IntoView {
    let mut next_counter_id = initial_dicts.len();

    let new_dicts = initial_dicts
        .into_iter()
        .map(|d| create_signal(d))
        .enumerate()
        .collect::<Vec<_>>();

    let (dicts, set_dicts) = create_signal(new_dicts);

    let add_dict = move |_| {
        let dict = Dictionary::Url(String::new());
        set_dicts.update(move |counters| counters.push((next_counter_id, create_signal(dict))));
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_dict>"New Dictionary"</button>
            <br/>
            <For
                each=dicts
                key=|dict| dict.0
                children=move |(id, (rdict, wdict))| {
                    view! {
                        <div class="dictionary_entry" style="display: inline-block">
                            <DictionaryRepresentation rdict=rdict wdict=wdict/>
                            <button class="remove" on:click=move |_| {
                                set_dicts
                                    .update(|dicts| {
                                        dicts.retain(|(dict_id, _)| dict_id != &id)
                                    });
                            }>
                                "X"
                            </button>
                        </div>
                        <br/>
                    }
                }
            />

        </div>
    }
}

#[component]
fn DictionaryRepresentation(
    rdict: ReadSignal<Dictionary>,
    wdict: WriteSignal<Dictionary>,
) -> impl IntoView {
    let is_file = move || matches!(rdict(), Dictionary::File(_, _));
    view! {
        <select
            id="dict_type"
            on:input=move |e| {
                match event_target_value(&e).as_str() {
                    "file" => {
                        if !is_file() {
                            wdict(Dictionary::File(String::new(), DictFileType::StarDict));
                        }
                    }
                    "url" => {
                        if is_file() {
                            wdict(Dictionary::Url(String::new()));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        >
            <option value="file" selected=is_file()>
                From file
            </option>
            <option value="url" selected=!is_file()>
                From server
            </option>
        </select>
        {move || match rdict() {
            Dictionary::Url(url) => {
                let (read_sig, write_sig) = create_signal(url);
                view! {
                    <SimpleTextSetting readsig=read_sig writesig=write_sig desc="url" name="url"/>
                }
                    .into_view()
            }
            Dictionary::File(filename, dict_type) => {
                let (read_filename, write_filename) = create_signal(filename);
                let is_stardict = matches!(dict_type, DictFileType::StarDict);
                view! {
                    <SimpleTextSetting
                        readsig=read_filename
                        writesig=write_filename
                        desc="File location"
                        name="file"
                    />
                    <button on:click=move |_| {
                        get_folder(write_filename);
                    }>open</button>
                    <select
                        id="file_type"
                        on:input=move |e| {
                            match event_target_value(&e).as_str() {
                                "stardict" => {
                                    if !is_stardict {
                                        wdict
                                            .update(|v| {
                                                if let Dictionary::File(fname, _) = v {
                                                    *v = Dictionary::File(
                                                        fname.to_string(),
                                                        DictFileType::StarDict,
                                                    );
                                                }
                                            })
                                    }
                                }
                                "delimiter" => {
                                    if is_stardict {
                                        wdict
                                            .update(|v| {
                                                if let Dictionary::File(fname, _) = v {
                                                    *v = Dictionary::File(
                                                        fname.to_string(),
                                                        DictFileType::TextSplitAt(String::from(":")),
                                                    );
                                                }
                                            })
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                    >
                        <option value="stardict" selected=is_stardict>
                            Stardict
                        </option>
                        <option value="delimiter" selected=!is_stardict>
                            Delimiter
                        </option>
                    </select>
                    {if let DictFileType::TextSplitAt(delim) = dict_type {
                        let (read_delim, write_delim) = create_signal(delim);
                        Some(
                            view! {
                                <SimpleTextSetting
                                    readsig=read_delim
                                    writesig=write_delim
                                    desc="Custom Delimiter"
                                    name="delim"
                                />
                            },
                        )
                    } else {
                        None
                    }}
                }
                    .into_view()
            }
        }}
    }
}
