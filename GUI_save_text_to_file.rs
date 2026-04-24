use eframe::egui;
use std::fs::File;
use std::io::Write;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Save Text to File",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wpisz tekst i zapisz do pliku");

            ui.text_edit_singleline(&mut self.text);

            if ui.button("Zapisz do pliku").clicked() {
                if let Ok(mut file) = File::create("D:/regex/src/output.txt") {
                    let _ = file.write_all(self.text.as_bytes());
                }
            }
        });
    }
}
