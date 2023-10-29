use leptos::{leptos_dom::logging::console_log, *};
use leptos_router::*;
use shared::*;
use tauri_sys::{dialog::FileDialogBuilder, tauri};

use crate::reader::{ReaderView, ReaderViewProps};
use crate::settings::{SettingsChanger, SettingsChangerProps};

mod reader;
mod settings;

fn main() {
    mount_to_body(|| view! { <App/> })
}

async fn get_settings() -> Settings {
    tauri::invoke("get_settings", &()).await.unwrap()
}

fn get_file(writer: WriteSignal<String>, folder: bool) {
    wasm_bindgen_futures::spawn_local(async move {
        console_log("getting file");
        let mut builder = FileDialogBuilder::new();
        let picker = if folder {
            builder.pick_folder().await
        } else {
            builder.pick_file().await
        };
        if let Ok(Some(v)) = picker {
            console_log(&format!("{:?}", v));
            writer(v.to_string_lossy().to_string());
        }
    })
}

#[component]
fn App() -> impl IntoView {
    let settings = create_resource(|| (), |_| async move { get_settings().await });
    view! {
        <Router>
            <style type="text/css">
                {move || settings().and_then(|s| s.css).unwrap_or_default()}
            </style>
            <nav>
                <NavBar/>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=move || ReaderView(ReaderViewProps { settings })/>
                    <Route path="/reader" view=move || ReaderView(ReaderViewProps { settings })/>
                    <Route
                        path="/settings"
                        view=move || SettingsChanger(SettingsChangerProps { settings })
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
