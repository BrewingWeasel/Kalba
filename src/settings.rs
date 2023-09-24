use serde::Deserialize;
use serde_derive::Serialize;
use std::fs;

use eframe::egui::{self, Context};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub model: String,
}

impl Settings {
    pub fn load_or_generate() -> (Self, bool) {
        let config_file = dirs::config_dir().unwrap().join("sakinyje.toml");
        match fs::read_to_string(config_file) {
            Ok(v) => (toml::from_str(&v).unwrap(), false),
            Err(_) => (
                Self {
                    model: String::new(),
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
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Current model location: ");
                    ui.text_edit_singleline(&mut self.model);
                });
                if ui.button("save").clicked() {
                    *show_settings = false;
                }
            });
    }
}
