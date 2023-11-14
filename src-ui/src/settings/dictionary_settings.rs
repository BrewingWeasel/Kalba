use leptos::*;

use shared::*;
type DictList = Vec<(usize, (ReadSignal<Dictionary>, WriteSignal<Dictionary>))>;
use crate::get_file;

#[component]
pub fn DictionaryList(
    dicts: ReadSignal<DictList>,
    set_dicts: WriteSignal<DictList>,
) -> impl IntoView {
    let mut next_dict_id = dicts().len();
    let add_dict = move |_| {
        let dict = Dictionary::Url(String::new());
        set_dicts.update(move |new_dicts| new_dicts.push((next_dict_id, create_signal(dict))));
        next_dict_id += 1;
    };

    view! {
        <div class="dicts">
            <h2 class="dicts_title">Dictionaries</h2>
            <button class="newdict" on:click=add_dict>
                "+"
            </button>
        </div>
        <div>
            <For
                each=dicts
                key=|dict| dict.0
                children=move |(id, (rdict, wdict))| {
                    view! {
                        <div class="dictionary_entry" style="display: inline-block">
                            <DictionaryRepresentation rdict=rdict wdict=wdict/>
                            <button
                                class="remove"
                                on:click=move |_| {
                                    set_dicts
                                        .update(|dicts| {
                                            dicts.retain(|(dict_id, _)| dict_id != &id)
                                        });
                                }
                            >

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
    view! {
        <div class="dropdown">
            <select
                id="dict_type"
                on:input=move |e| {
                    match event_target_value(&e).as_str() {
                        "file" => {
                            if !matches!(rdict(), Dictionary::File(_, _)) {
                                wdict(Dictionary::File(String::new(), DictFileType::StarDict));
                            }
                        }
                        "url" => {
                            if !matches!(rdict(), Dictionary::Url(_)) {
                                wdict(Dictionary::Url(String::new()));
                            }
                        }
                        "command" => {
                            if !matches!(rdict(), Dictionary::Command(_)) {
                                wdict(Dictionary::Command(String::new()));
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            >

                <option value="file" selected=matches!(rdict(), Dictionary::File(_, _))>
                    From file
                </option>
                <option value="url" selected=matches!(rdict(), Dictionary::Url(_))>
                    From server
                </option>
                <option value="command" selected=matches!(rdict(), Dictionary::Command(_))>
                    From command
                </option>
            </select>
        </div>
        {move || match rdict() {
            Dictionary::Url(url) => {
                let (read_sig, write_sig) = create_signal(url);
                view! {
                    <div class="labeledinput">
                        <label for="url">Url</label>
                        <input
                            id="url"
                            type="text"
                            on:input=move |ev| {
                                write_sig(event_target_value(&ev));
                            }

                            on:change=move |_| {
                                wdict.update(|v| { *v = Dictionary::Url(read_sig()) })
                            }

                            prop:value=read_sig
                        />
                    </div>
                }
                    .into_view()
            }
            Dictionary::Command(url) => {
                let (read_sig, write_sig) = create_signal(url);
                view! {
                    <div class="labeledinput">
                        <label for="command">Command</label>
                        <input
                            id="command"
                            type="text"
                            on:input=move |ev| {
                                write_sig(event_target_value(&ev));
                            }

                            on:change=move |_| {
                                wdict.update(|v| { *v = Dictionary::Command(read_sig()) })
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
                        <label for="filename">File location</label>
                        <input
                            id="filename"
                            type="text"
                            on:input=move |ev| {
                                write_filename(event_target_value(&ev));
                            }

                            on:change=move |_| {
                                wdict
                                    .update(|v| {
                                        if let Dictionary::File(_, file_type) = v {
                                            *v = Dictionary::File(read_filename(), file_type.clone());
                                        }
                                    });
                            }

                            prop:value=read_filename
                        />
                    </div>

                    <button
                        class="selectfile"
                        on:click=move |_| {
                            get_file(write_filename, false);
                        }
                    >

                        browse
                    </button>
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
                                    <label for="delim">Delimiter</label>
                                    <input
                                        id="delim"
                                        type="text"
                                        on:input=move |ev| {
                                            write_delim(event_target_value(&ev));
                                        }

                                        on:change=move |_| {
                                            wdict
                                                .update(|v| {
                                                    if let Dictionary::File(file_name, _) = v {
                                                        *v = Dictionary::File(
                                                            file_name.clone(),
                                                            DictFileType::TextSplitAt(read_delim()),
                                                        );
                                                    }
                                                });
                                        }

                                        prop:value=read_delim
                                    />
                                </div>
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
