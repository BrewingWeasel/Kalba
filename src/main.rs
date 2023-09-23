use arboard::Clipboard;
use dictionary::get_def;
use eframe::{
    egui::{self, Label, RichText, Sense, Separator, TextStyle},
    epaint::{Color32, Vec2},
};
use language_parsing::{get_words, Word};
mod dictionary;
mod language_parsing;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(380.0, 250.0)),
        decorated: false,
        always_on_top: true,
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    words: Vec<Word>,
    sentence: String,
    current: Option<usize>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut clipboard = Clipboard::new().unwrap();
        let selected = clipboard.get_text().unwrap();
        Self {
            words: get_words(&selected),
            sentence: selected,
            current: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();
            style.text_styles.get_mut(&TextStyle::Body).unwrap().size = 16.0;
            style.spacing.item_spacing = Vec2::new(0.0, 0.0);
            ctx.set_style(style);
            ui.horizontal_wrapped(|ui| {
                for (i, val) in self.words.iter().enumerate() {
                    if val.clickable {
                        // TODO: seperate clickable from punctuation etc
                        ui.add_space(5.0)
                        // TODO: also shouldn't be hardcoded, should be based on font size or smth
                    }
                    if ui
                        .add(
                            Label::new(
                                RichText::from(&val.text)
                                    .color(Color32::from_rgb(230, 140, 210))
                                    .text_style(egui::TextStyle::Body),
                            )
                            .sense(Sense::click()),
                        )
                        .clicked()
                    {
                        println!("{}", val.lemma);
                        self.current = Some(i);
                    }
                }
            });

            if let Some(i) = self.current {
                ui.vertical_centered(|ui| {
                    ui.add(Separator::default().spacing(9.0));
                    ui.add(Label::new(
                        RichText::from(&self.words[i].lemma)
                            .color(Color32::WHITE)
                            .underline()
                            .strong()
                            .text_style(egui::TextStyle::Heading),
                    ));
                    ui.add_space(5.0);
                    ui.add(Label::new(
                        RichText::from(get_def(&self.words[i].lemma))
                            .color(Color32::from_rgb(210, 170, 250))
                            .text_style(egui::TextStyle::Body),
                    ));
                    ui.add_space(10.0);
                    if ui.button("Export sentence").clicked() {
                        println!(
                            "exported {}, focusing on {}",
                            self.sentence, self.words[i].lemma
                        );
                    }
                });
            }
        });
    }
}
