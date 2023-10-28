use leptos::{
    leptos_dom::logging::{console_error, console_log},
    *,
};
use serde::Serialize;
use shared::*;
use tauri_sys::tauri;

use crate::get_file;

type DictList = Vec<(usize, (ReadSignal<Dictionary>, WriteSignal<Dictionary>))>;

#[derive(Serialize)]
pub struct SettingsSaver {
    pub settings: Settings,
}

fn save_settings(settings: Settings) {
    wasm_bindgen_futures::spawn_local(async move {
        console_log("saving settings");
        match tauri::invoke("write_settings", &SettingsSaver { settings }).await {
            Err(e) => console_error(&e.to_string()),
            Ok(()) => (),
        }
    })
}

#[component]
pub fn SettingsChanger(settings: Resource<(), Settings>) -> impl IntoView {
    let old_settings = if let Some(s) = settings() {
        s
    } else {
        return view! { <p>Unable to load settings</p> }.into_view();
    };

    let (model, set_model) = create_signal(old_settings.model);
    let (deck, set_deck) = create_signal(old_settings.deck);
    let (note, set_note) = create_signal(old_settings.note_type);

    let new_dicts = old_settings
        .dicts
        .clone()
        .into_iter()
        .map(|d| create_signal(d))
        .enumerate()
        .collect::<Vec<_>>();

    let (dicts, set_dicts) = create_signal(new_dicts);

    view! {
        <div class="settings">
        <h2>Grammatical parsing</h2>
        <div class="model_selection">
            <SimpleTextSetting readsig=model writesig=set_model name="model" desc="SpaCy Model"/>
            <button class="selectfile" on:click=move |_| {
                get_file(set_model, true);
            }>browse</button>
        </div>
        <hr/>
        <h2>Anki Settings</h2>
        <SimpleTextSetting readsig=deck writesig=set_deck name="deck" desc="Anki Deck"/>
        <br/>
        <SimpleTextSetting readsig=note writesig=set_note name="note" desc="Note type"/>

        <hr/>
        <DictionaryList dicts=dicts set_dicts=set_dicts/>

        <hr/>

        <button class="parsebutton" on:click=move |_| {
            settings
                .update(|v| {
                    let updater = v.as_mut().unwrap();
                    // TODO: maybe make a macro for this
                    updater.model = model();
                    updater.deck = deck();
                    updater.note_type = note();
                    updater.dicts = dicts().iter().map(|(_, (r, _))| r()).collect();
                });
            save_settings(settings().unwrap());
        }>save</button>
        </div>
    }
    .into_view()
}

#[component]
fn SimpleTextSetting(
    readsig: ReadSignal<String>,
    writesig: WriteSignal<String>,
    name: &'static str,
    desc: &'static str,
) -> impl IntoView {
    view! {
        <div class="labeledinput">
        <label for=name>{desc}</label>
        <input
            id=name
            type="text"
            on:input=move |ev| {
                writesig(event_target_value(&ev));
            }

            prop:value=readsig
        /></div>
    }
}

#[component]
fn DictionaryList(dicts: ReadSignal<DictList>, set_dicts: WriteSignal<DictList>) -> impl IntoView {
    let mut next_dict_id = dicts().len();
    let add_dict = move |_| {
        let dict = Dictionary::Url(String::new());
        set_dicts.update(move |new_dicts| new_dicts.push((next_dict_id, create_signal(dict))));
        next_dict_id += 1;
    };

    view! {
        <div class="dicts">
            <h2 class="dicts_title">Dictionaries</h2>
            <button class="newdict" on:click=add_dict>"+"</button>
        </div>
        <div>
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
                                "x"
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
        <div class="dropdown">
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
        </div>
        {move || match rdict() {
            Dictionary::Url(url) => {
                let (read_sig, write_sig) = create_signal(url);
                view! {
                    <div class="labeledinput">
                    <label for="url">Url</label> // TODO: make generic function for this
                    <input
                        id="url"
                        type="text"
                        on:input=move |ev| {
                            write_sig(event_target_value(&ev));
                        }
                        on:change=move |_| {
                            wdict.update(|v| {
                                *v = Dictionary::Url(read_sig())
                            })
                        }
                        prop:value=read_sig
                    />
                    </div>
                }
                    .into_view()
            }
            Dictionary::File(filename, dict_type) => {
                let (read_filename, write_filename) = create_signal(filename);
                let is_stardict = matches!(dict_type, DictFileType::StarDict);
                view! {
                    <div class="labeledinput">
                    <label for="filename">File location</label> // TODO: make generic function for this
                    <input
                        id="filename"
                        type="text"
                        on:input=move |ev| {
                            write_filename(event_target_value(&ev));
                        }
                        on:change=move |_| {
                            wdict.update(|v| {
                                if let Dictionary::File(_, file_type) = v {
                                    *v = Dictionary::File(read_filename(), file_type.clone()); // TODO:
                                    // no clone here
                                }
                            });
                        }
                        prop:value=read_filename
                    />
                    </div>

                    <button class="selectfile" on:click=move |_| {
                        get_file(write_filename, false);
                    }>browse</button>
                    <div class="dropdown">
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
                    </div>
                    {if let DictFileType::TextSplitAt(delim) = dict_type {
                        let (read_delim, write_delim) = create_signal(delim);
                        Some(
                            view! {
                                <div class="labeledinput">
                                <label for="delim">Delimiter</label> // TODO: make generic function for this
                                <input
                                    id="delim"
                                    type="text"
                                    on:input=move |ev| {
                                        write_delim(event_target_value(&ev));
                                    }
                                    on:change=move |_| {
                                        wdict.update(|v| {
                                            if let Dictionary::File(file_name, _) = v {
                                                *v = Dictionary::File(file_name.clone(), DictFileType::TextSplitAt(read_delim())); // TODO:
                                                // no clone here
                                            }
                                        });
                                    }
                                    prop:value=read_delim
                                /></div>
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
