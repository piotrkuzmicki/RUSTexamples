use eframe::egui;
use std::fs;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSV Drag & Drop",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    dropped_file: Option<PathBuf>,
    show_error: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            dropped_file: None,
            show_error: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- OBSŁUGA DRAG & DROP ---
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files = ctx.input(|i| i.raw.dropped_files.clone());
            if let Some(file) = files.first() {
                if let Some(path) = &file.path {
                    self.dropped_file = Some(path.clone());
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drag & Drop CSV File");

            ui.label("Przeciągnij plik CSV tutaj:");

            ui.separator();

            if let Some(path) = &self.dropped_file {
                ui.label(format!("Wybrany plik: {}", path.display()));

                if ui.button("SAVE").clicked() {
                    // Walidacja rozszerzenia
                    if path.extension().and_then(|e| e.to_str()) != Some("csv") {
                        self.show_error = true;
                    } else {
                        let dest = "D:/gui2/src/file2.csv";
                        let _ = fs::copy(path, dest);
                    }
                }
            } else {
                ui.label("Brak pliku");
            }

            // --- POPUP BŁĘDU ---
            if self.show_error {
                egui::Window::new("Error")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("This is not a CSV file. Upload another file.");
                        if ui.button("OK").clicked() {
                            self.show_error = false;
                        }
                    });
            }
        });
    }
}
