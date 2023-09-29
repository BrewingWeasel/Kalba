use serde::Deserialize;
use serde_derive::Serialize;
use std::fs;

use eframe::egui::{self, Context, TextEdit};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub model: String,
    pub dicts: Vec<Dictionary>,
    pub to_remove: Option<usize>,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum Dictionary {
    File(String), // Eventually these will have specific things
    Url(String),  // TODO: implement url
}

impl Settings {
    pub fn load_or_generate() -> (Self, bool) {
        let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");
        match fs::read_to_string(config_file) {
            Ok(v) => (toml::from_str(&v).unwrap(), false),
            Err(_) => (
                Self {
                    model: String::new(),
                    dicts: Vec::new(),
                    to_remove: None,
                },
                true,
            ),
        }
    }

    pub fn save(&self) {
        let conts = toml::to_string_pretty(&self).unwrap();
        let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");
        fs::write(config_file, conts).expect("Failure saving settings");
    }

    pub fn make_window(&mut self, ctx: &Context, show_settings: &mut bool) {
        if let Some(i) = self.to_remove {
            self.dicts.remove(i);
            self.to_remove = None;
        }
        egui::Window::new("Settings")
            .auto_sized()
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Current model location: ");
                    ui.add(TextEdit::singleline(&mut self.model).desired_width(400.0))
                });
                ui.horizontal(|ui| {
                    ui.label("Dictionaries:");
                    if ui.button("new dict").clicked() {
                        self.dicts.push(Dictionary::File(String::new()))
                    }
                });
                for (i, dict) in self.dicts.iter_mut().enumerate() {
                    let selected = match dict {
                        Dictionary::File(_) => "File",
                        Dictionary::Url(_) => "URL",
                    };
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_label(format!("Dict {i}"))
                            .selected_text(selected)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(dict, Dictionary::File(String::new()), "File");
                                ui.selectable_value(dict, Dictionary::Url(String::new()), "URL");
                            });
                        ui.add_space(5.0);
                        match dict {
                            Dictionary::File(f) => {
                                ui.add(TextEdit::singleline(f).desired_width(400.0))
                            }
                            Dictionary::Url(url) => {
                                ui.add(TextEdit::singleline(url).desired_width(400.0))
                            }
                        };
                        if ui.button("X").clicked() {
                            self.to_remove = Some(i);
                        }
                    });
                }
                if ui.button("save").clicked() {
                    *show_settings = false;
                }
            });
    }
}
