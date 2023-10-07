use serde::Deserialize;
use serde_derive::Serialize;
use std::fs;

use eframe::egui::{self, Context, TextEdit};

use crate::dictionary::DictFileType;

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub model: String,
    pub dicts: Vec<Dictionary>,
    pub to_remove: Option<usize>,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum Dictionary {
    File(String, DictFileType), // Eventually these will have specific things
    Url(String),                // TODO: implement url
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
                    ui.label("Current spacy model: ");
                    ui.add(TextEdit::singleline(&mut self.model).desired_width(400.0));
                    if ui.button("select file").clicked() {
                        if let Some(f) = rfd::FileDialog::new().pick_folder() {
                            self.model = f.to_str().unwrap().to_owned();
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Dictionaries:");
                    if ui.button("new dict").clicked() {
                        self.dicts
                            .push(Dictionary::File(String::new(), DictFileType::StarDict))
                    }
                });
                for (i, dict) in self.dicts.iter_mut().enumerate() {
                    let selected = match dict {
                        Dictionary::File(_, _) => "File",
                        Dictionary::Url(_) => "URL",
                    };
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_label(format!("Dict {i}"))
                            .selected_text(selected)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    dict,
                                    Dictionary::File(String::new(), DictFileType::StarDict),
                                    "Local Dictionary",
                                );
                                ui.selectable_value(dict, Dictionary::Url(String::new()), "URL");
                            });
                        ui.add_space(5.0);
                        match dict {
                            Dictionary::File(file, dict_type) => {
                                ui.add(TextEdit::singleline(file).desired_width(400.0));

                                let selected = match dict_type {
                                    DictFileType::StarDict => "StarDict",
                                    DictFileType::TextSplitAt(_) => "Custom",
                                };

                                egui::ComboBox::from_id_source(i)
                                    .selected_text(selected)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            dict_type,
                                            DictFileType::StarDict,
                                            "StarDict",
                                        );
                                        ui.selectable_value(
                                            dict_type,
                                            DictFileType::TextSplitAt(String::from("=")),
                                            "Custom Type",
                                        );
                                    });

                                if let DictFileType::TextSplitAt(delimiter) = dict_type {
                                    ui.add(TextEdit::singleline(delimiter).desired_width(40.0));
                                }

                                if ui.button("select file").clicked() {
                                    let file_picker = match dict_type {
                                        DictFileType::TextSplitAt(_) => {
                                            rfd::FileDialog::new().pick_file()
                                        }
                                        DictFileType::StarDict => rfd::FileDialog::new()
                                            .add_filter("StarDict", &["ifo"])
                                            .pick_file(),
                                    };
                                    if let Some(selected_file) = file_picker {
                                        *file = selected_file.to_str().unwrap().to_owned();
                                    }
                                }
                            }
                            Dictionary::Url(url) => {
                                ui.add(TextEdit::singleline(url).desired_width(400.0));
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
