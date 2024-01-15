use crate::settings::{
    dictionary_settings::DictionaryList,
    shared_ui::*,
    tauri_communicate::*,
    wordknowledge_settings::{AnkiInfo, TemplateList, WordKnowledgeList},
};
use std::collections::HashMap;

use leptos::*;
use shared::*;

use crate::get_file;
mod dictionary_settings;
mod shared_ui;
mod tauri_communicate;
mod wordknowledge_settings;

#[component]
pub fn SettingsChanger(settings: Resource<(), Settings>) -> impl IntoView {
    let old_settings = if let Some(s) = settings() {
        s
    } else {
        return view! { <p>Unable to load settings</p> }.into_view();
    };

    let info = AnkiInfo {
        decks: create_resource(|| (), |_| async move { get_all_x_names("deck").await }),
        templates: create_resource(|| (), |_| async move { get_all_x_names("note").await }),
    };

    let (model, set_model) = create_signal(old_settings.model);
    let (deck, set_deck) = create_signal(old_settings.deck);
    let (note, set_note) = create_signal(old_settings.note_type);
    let (note_fields, set_note_fields) = create_signal(old_settings.note_fields);

    let (css, set_css) = create_signal(old_settings.css.unwrap_or_default());
    let (commands, set_commands) =
        create_signal(old_settings.to_run.unwrap_or_default().join("\n"));

    let new_dicts = old_settings
        .dicts
        .into_iter()
        .map(create_signal)
        .enumerate()
        .collect::<Vec<_>>();

    let new_templates: TemplateList = old_settings
        .anki_parser
        .unwrap_or_default()
        .into_iter()
        .map(|v| create_signal((v.0, v.1.into_iter().map(create_signal).collect::<Vec<_>>())))
        .enumerate()
        .collect::<Vec<_>>();

    let (dicts, set_dicts) = create_signal(new_dicts);
    let (templates, set_templates) = create_signal(new_templates);

    let save_settings_button = move || {
        settings.update(|v| {
            let updater = v.as_mut().unwrap();
            updater.model = model();
            updater.deck = deck();
            updater.note_type = note();
            updater.note_fields = note_fields();
            updater.css = if css().is_empty() { None } else { Some(css()) };
            updater.to_run = if commands().is_empty() {
                None
            } else {
                Some(commands().split('\n').map(|v| v.to_string()).collect())
            };
            updater.dicts = dicts().iter().map(|(_, (r, _))| r()).collect();
            let mut updated_templates = HashMap::new();
            for (_, (readdeck, _)) in templates().iter() {
                let deckname = readdeck().0;
                let mut notes = Vec::new();
                for (readnote, _) in readdeck().1 {
                    notes.push(readnote())
                }
                updated_templates.insert(deckname, notes.into_iter().collect());
            }
            updater.anki_parser = Some(updated_templates);
        });
        save_settings(settings().unwrap());
    };

    view! {
        <div class="settings">
            <h2>Grammatical parsing</h2>
            <div class="model_selection">
                <SimpleTextSetting
                    readsig=model
                    writesig=set_model
                    name="model"
                    desc="SpaCy Model"
                />
                <button
                    class="selectfile"
                    on:click=move |_| {
                        get_file(set_model, true);
                    }
                >

                    browse
                </button>
            </div>
            <hr/>
            <h2>Anki Settings</h2>
            <SimpleDropDown
                readsig=deck
                writesig=set_deck
                name="deck"
                desc="Anki Deck"
                options=info.decks
            />
            <br/>
            <SimpleDropDown
                readsig=note
                writesig=set_note
                name="note"
                desc="Note type"
                options=info.templates
            />
            <br/>
            <SimpleTextAreaSetting
                readsig=note_fields
                writesig=set_note_fields
                name="notefield"
                desc="Note Fields"
            />

            <hr/>
            <DictionaryList dicts=dicts set_dicts=set_dicts/>

            <hr/>
            <WordKnowledgeList templates=templates set_templates=set_templates info=info/>

            <hr/>
            <h2>Styling</h2>
            <SimpleTextAreaSetting readsig=css writesig=set_css name="css" desc="Css Styling"/>
            <hr/>

            <hr/>
            <h2>Automatically run commands</h2>
            <p>Be very very careful with what you put in here</p>
            <SimpleTextAreaSetting
                readsig=commands
                writesig=set_commands
                name="commands"
                desc="Commands to run on launch"
            />
            <hr/>

            <button class="parsebutton" on:click=move |_| { save_settings_button() }>
                save
            </button>
        </div>
    }
    .into_view()
}
