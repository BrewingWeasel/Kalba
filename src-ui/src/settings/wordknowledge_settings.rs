use leptos::*;
use shared::NoteToWordHandling;

use crate::settings::{
    get_template_fields,
    shared_ui::{SimpleDropDown, SimpleTextSetting},
};

type AnkiDeck = (
    String,
    Vec<(
        ReadSignal<(String, NoteToWordHandling)>,
        WriteSignal<(String, NoteToWordHandling)>,
    )>,
);
pub type TemplateList = Vec<(usize, (ReadSignal<AnkiDeck>, WriteSignal<AnkiDeck>))>;

#[derive(Clone, Copy)]
pub struct AnkiInfo {
    pub decks: Resource<(), Vec<String>>,
    pub templates: Resource<(), Vec<String>>,
}

#[component]
pub fn WordKnowledgeList(
    templates: ReadSignal<TemplateList>,
    set_templates: WriteSignal<TemplateList>,
    info: AnkiInfo,
) -> impl IntoView {
    let mut next_templ_id = templates().len();
    let add_template = move |_| {
        let new_template = (String::new(), Vec::new());
        set_templates
            .update(move |templs| templs.push((next_templ_id, create_signal(new_template))));
        next_templ_id += 1;
    };
    let each_template = move |(id, (rtempl, wtempl))| {
        view! {
            <div class="deck_templates">
                <button
                    class="remove"
                    on:click=move |_| {
                        set_templates
                            .update(|templ| { templ.retain(|(templ_id, _)| templ_id != &id) });
                    }
                >

                    "x"
                </button>
                <IndividualDeckRepresentation rtempl=rtempl wtempl=wtempl info=info/>
                <hr/>
            </div>
        }
    };
    view! {
        <div class="dicts">
            <h2 class="dicts_title">Word Knowledge</h2>
            <button class="smallernewbutton" on:click=add_template>
                "connect new deck"
            </button>
        </div>
        <div class="all_templates">
            <For each=templates key=|templ| templ.0 children=each_template/>

        </div>
    }
}

#[component]
fn IndividualDeckRepresentation(
    rtempl: ReadSignal<AnkiDeck>,
    wtempl: WriteSignal<AnkiDeck>,
    info: AnkiInfo,
) -> impl IntoView {
    let add_note_template = move |_| {
        let new_template = NoteToWordHandling {
            field_to_use: String::new(),
            only_first_word_or_line: false,
            remove_everything_in_parens: false,
            tags_wanted: Vec::new(),
        };
        wtempl.update(move |templs| templs.1.push(create_signal((String::new(), new_template))));
    };
    view! {
        <SimpleDropDown
            readsig=move || rtempl().0
            writesig=move |x| wtempl.update(|v| v.0 = x)
            name="deck"
            desc="Anki Deck"
            options=info.decks
        />
        <button class="smallernewbutton" on:click=add_note_template>
            "new note type"
        </button>
        <For
            each=move || rtempl().1
            key=|templ| templ.0
            children=move |(rnote, wnote)| {
                view! { <AnkiNoteParsing rnote=rnote wnote=wnote info=info/> }
            }
        />
    }
}

#[component]
fn AnkiNoteParsing(
    rnote: ReadSignal<(String, NoteToWordHandling)>,
    wnote: WriteSignal<(String, NoteToWordHandling)>,
    info: AnkiInfo,
) -> impl IntoView {
    let fields_resource = create_resource(move || rnote().0, get_template_fields);
    view! {
        <hr/>
        <div class="individualnote">
            <SimpleDropDown
                readsig=move || rnote().0
                writesig=move |x| wnote.update(|v| v.0 = x)
                name="template"
                desc="Anki Template"
                options=info.templates
            />
            <SimpleDropDown
                readsig=move || rnote().1.field_to_use
                writesig=move |x| wnote.update(|v| v.1.field_to_use = x)
                name="field"
                desc="Field"
                options=fields_resource
            />

            <div class="labeledcheckbox">
                <label for="firstword">Only take first word/line</label>
                <input
                    id="firstword"
                    type="checkbox"
                    on:change=move |ev| {
                        wnote.update(|v| v.1.only_first_word_or_line = event_target_checked(&ev))
                    }

                    prop:value=move || rnote().1.only_first_word_or_line
                />
            </div>

            <div class="labeledcheckbox">
                <label class="checkboxlabel" for="removeparens">
                    Remove everything in parentheses
                </label>
                <input
                    id="removeparens"
                    class="checkbox"
                    type="checkbox"
                    on:change=move |ev| {
                        wnote
                            .update(|v| v.1.remove_everything_in_parens = event_target_checked(&ev))
                    }

                    prop:value=move || rnote().1.remove_everything_in_parens
                />
            </div>

            <SimpleTextSetting
                readsig=move || rnote().1.tags_wanted.join(" ")
                writesig=move |inp| wnote.update(|v| v.1.tags_wanted = inp.split_whitespace().map(|t| t.to_string()).collect())
                name="tags"
                desc="Tags required"
            />
        </div>
    }
}
