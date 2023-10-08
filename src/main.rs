use std::{
    collections::{hash_map, HashMap},
    process,
};

use arboard::Clipboard;
use dictionary::get_defs;
use eframe::{
    egui::{self, Button, Label, RichText, Sense, Separator, TextEdit, TextStyle},
    emath::Align,
    epaint::{Color32, Vec2},
};
use language_parsing::{get_words, Word};
use settings::Settings;

use crate::add_to_anki::add_to_anki;

mod add_to_anki;
mod dictionary;
mod language_parsing;
mod settings;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(380.0, 250.0)),
        decorated: false,
        always_on_top: true,
        // resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Sakinyje",
        options,
        Box::new(|_cc| Box::<Sakinyje>::default()),
    )
}

struct Sakinyje {
    words: Vec<Word>,
    sentence: String,
    current: Option<usize>,
    definitions: HashMap<usize, Vec<String>>,
    settings: Settings,
    show_settings: bool,
    error_to_show: Option<String>,
    get_def: bool,
}

impl Default for Sakinyje {
    fn default() -> Self {
        let mut clipboard = Clipboard::new().unwrap();
        let selected = clipboard.get_text().unwrap();

        let (settings, show_settings) = Settings::load_or_generate();

        let (words, error_to_show) = match get_words(&selected, &settings.model) {
            Ok(v) => (v, None),
            Err(e) => (Vec::new(), Some(e.to_string())),
        };

        Self {
            words,
            sentence: selected,
            current: None,
            definitions: HashMap::new(),
            settings,
            show_settings,
            error_to_show,
            get_def: true,
        }
    }
}

impl Sakinyje {
    fn update(&mut self) {
        let mut clipboard = Clipboard::new().unwrap();
        self.sentence = clipboard.get_text().unwrap();
        self.definitions = HashMap::new();
        self.current = None;

        let (words, error_to_show) = match get_words(&self.sentence, &self.settings.model) {
            Ok(v) => (v, None),
            Err(e) => (Vec::new(), Some(e.to_string())),
        };

        self.get_def = true;
        self.words = words;
        self.error_to_show = error_to_show;
    }
}

impl eframe::App for Sakinyje {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.settings.save();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();
            style.text_styles.get_mut(&TextStyle::Body).unwrap().size = 16.0;
            style.spacing.item_spacing = Vec2::new(0.0, 0.0);
            ctx.set_style(style);

            ui.horizontal(|ui| {
                if ui.button("settings").clicked() {
                    self.show_settings = true;
                }
                if ui.button("update").clicked() {
                    self.update();
                }
            });

            if self.show_settings {
                self.settings.make_window(ctx, &mut self.show_settings);
            } else if self.error_to_show.is_some() {
                egui::Window::new("Settings")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(self.error_to_show.as_ref().unwrap());
                        if ui.button("ok").clicked() {
                            self.error_to_show = None;
                        }
                    });
            }

            ui.horizontal_wrapped(|ui| {
                for (i, val) in self.words.iter().enumerate() {
                    if val.clickable {
                        // TODO: seperate clickable from punctuation etc
                        ui.add_space(5.0)
                        // TODO: also shouldn't be hardcoded, should be based on font size or smth
                    }

                    let color = match val.morph.as_deref() {
                        Some("Nom") => Color32::from_rgb(246, 193, 119),
                        Some("Gen") => Color32::from_rgb(235, 111, 146),
                        Some("Acc") => Color32::from_rgb(234, 154, 151),
                        Some("Ins") => Color32::from_rgb(62, 143, 176),
                        Some("Loc") => Color32::from_rgb(156, 207, 216),
                        Some("Voc") => Color32::from_rgb(196, 167, 231),
                        _ => Color32::from_rgb(224, 222, 244),
                    };

                    let word = ui.add(
                        Label::new(
                            RichText::from(&val.text)
                                .color(color)
                                .text_style(egui::TextStyle::Body),
                        )
                        .sense(Sense::click()),
                    );

                    if word.clicked() && val.clickable {
                        println!("{}", val.lemma);
                        self.current = Some(i);
                        self.get_def = !ui.input(|i| i.modifiers.shift);
                    }
                }
            });

            if let Some(i) = self.current {
                ui.vertical_centered(|ui| {
                    ui.add(Separator::default().spacing(9.0));

                    let lemma_place = ui.add(
                        TextEdit::singleline(&mut self.words[i].lemma)
                            .text_color(Color32::WHITE)
                            .horizontal_align(Align::Center)
                            .frame(false)
                            .font(TextStyle::Heading),
                    );
                    if lemma_place.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.definitions
                            .insert(i, get_defs(&self.words[i].lemma, &self.settings.dicts));
                        self.get_def = true;
                    }

                    ui.add_space(5.0);

                    if self.get_def {
                        if let hash_map::Entry::Vacant(e) = self.definitions.entry(i) {
                            e.insert(get_defs(&self.words[i].lemma, &self.settings.dicts));
                        }
                        let defs = self.definitions.get_mut(&i).unwrap();
                        // Not sure why the below doesn't work, but it doesn't
                        //
                        // let defs = self
                        //     .definitions
                        //     .entry(i)
                        //     .or_insert(get_defs(&self.words[i].lemma, &self.settings.dicts));

                        for def in defs.iter_mut() {
                            ui.add(
                                TextEdit::multiline(def)
                                    .desired_rows(1)
                                    .desired_width(f32::INFINITY)
                                    .text_color(Color32::from_rgb(210, 170, 250))
                                    .horizontal_align(Align::Center)
                                    .frame(false)
                                    .font(TextStyle::Body),
                            );
                            ui.add_space(5.0);
                        }
                    }

                    ui.add_space(8.0);

                    if ui
                        .add(
                            Button::new(
                                RichText::from("Export sentence")
                                    .color(Color32::from_rgb(150, 250, 230))
                                    .strong()
                                    .text_style(egui::TextStyle::Heading),
                            )
                            .rounding(10.0),
                        )
                        .clicked()
                    {
                        self.settings.save();
                        if let Err(e) = add_to_anki(
                            &self.sentence,
                            &self.words[i].lemma,
                            self.definitions.get_mut(&i).unwrap(),
                            &self.settings,
                        ) {
                            eprintln!("{}", e);
                        }
                        println!(
                            "exported {}, focusing on {}",
                            self.sentence, self.words[i].lemma
                        );
                        process::exit(0);
                    }
                });
            }
        });
    }
}
