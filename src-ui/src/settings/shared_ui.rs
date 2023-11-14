#![allow(unused_braces)]
use leptos::*;

#[component]
pub fn SimpleTextSetting<Read, Write>(
    readsig: Read,
    mut writesig: Write,
    name: &'static str,
    desc: &'static str,
) -> impl IntoView
where
    Read: Fn() -> String + 'static + Copy,
    Write: FnMut(String) + 'static + Copy,
{
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
            />
        </div>
    }
}

#[component]
pub fn SimpleDropDown<Read, Write, OptionsGetter>(
    readsig: Read,
    mut writesig: Write,
    name: &'static str,
    desc: &'static str,
    options: OptionsGetter,
) -> impl IntoView
where
    Read: Fn() -> String + 'static + Copy,
    Write: FnMut(String) + 'static + Copy,
    OptionsGetter: Fn() -> Option<Vec<String>> + 'static,
{
    let get_options = move |opts: &Vec<String>| -> View {
        opts.iter()
            .map(|x| {
                view! {
                    <option value=x selected=readsig() == *x>
                        {x}
                    </option>
                }
                .into_view()
            })
            .collect_view()
    };

    let get_dropdown = move |opts: Vec<String>| {
        view! {
            <div class="dropdown">
                <label for=name>{desc}</label>
                <select
                    id=name
                    on:input=move |ev| {
                        writesig(event_target_value(&ev));
                    }

                    prop:value=readsig
                >
                    {move || get_options(&opts)}
                </select>
            </div>
        }
    };

    view! { {move || { options().map(get_dropdown) }} }
}

#[component]
pub fn SimpleTextAreaSetting(
    readsig: ReadSignal<String>,
    writesig: WriteSignal<String>,
    name: &'static str,
    desc: &'static str,
) -> impl IntoView {
    view! {
        <div class="labeledinput">
            <label for=name>{desc}</label>
            <textarea
                id=name
                type="text"
                on:input=move |ev| {
                    writesig(event_target_value(&ev));
                }

                prop:value=readsig
            ></textarea>
        </div>
    }
}
