use eframe::egui;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSV Compare",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    file1: Option<PathBuf>,
    file2: Option<PathBuf>,
    show_error: bool,
    error_message: String,
    show_success: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file1: None,
            file2: None,
            show_error: false,
            error_message: String::new(),
            show_success: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- DRAG & DROP FILES ---
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let dropped = ctx.input(|i| i.raw.dropped_files.clone());

            for file in dropped {
                if let Some(path) = file.path {
                    if path.extension().and_then(|e| e.to_str()) != Some("csv") {
                        self.show_error = true;
                        self.error_message = "Only CSV files are allowed".to_string();
                        continue;
                    }

                    if self.file1.is_none() {
                        self.file1 = Some(path);
                    } else if self.file2.is_none() {
                        self.file2 = Some(path);
                    }
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drag & Drop Two CSV Files");
            ui.separator();

            ui.label("File 1:");
            ui.label(self.file1.as_ref().map(|p| p.display().to_string()).unwrap_or("None".into()));

            ui.separator();

            ui.label("File 2:");
            ui.label(self.file2.as_ref().map(|p| p.display().to_string()).unwrap_or("None".into()));

            ui.separator();

            if self.file1.is_some() && self.file2.is_some() {
                if ui.button("COMPARE CSV").clicked() {
                    match compare_csv(
                        self.file1.as_ref().unwrap(),
                        self.file2.as_ref().unwrap(),
                    ) {
                        Ok(_) => self.show_success = true,
                        Err(err) => {
                            self.show_error = true;
                            self.error_message = err;
                        }
                    }
                }
            } else {
                ui.label("Drop two CSV files above");
            }

            // --- POPUP ERROR ---
            if self.show_error {
                egui::Window::new("Error")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(&self.error_message);
                        if ui.button("OK").clicked() {
                            self.show_error = false;
                        }
                    });
            }

            // --- POPUP SUCCESS ---
            if self.show_success {
                egui::Window::new("Success")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Differences saved to diff.txt");
                        if ui.button("OK").clicked() {
                            self.show_success = false;
                        }
                    });
            }
        });
    }
}

// ------------------------------------------------------------
// COMPARE TWO CSV FILES AND SAVE DIFFERENCES
// ------------------------------------------------------------
fn compare_csv(file1: &PathBuf, file2: &PathBuf) -> Result<(), String> {
    let f1 = File::open(file1)
        .map_err(|_| format!("Cannot open file: {}", file1.display()))?;
    let f2 = File::open(file2)
        .map_err(|_| format!("Cannot open file: {}", file2.display()))?;

    let r1 = BufReader::new(f1).lines();
    let r2 = BufReader::new(f2).lines();

    let mut diff = File::create("diff.txt")
        .map_err(|_| "Cannot create diff.txt".to_string())?;

    let mut differences_found = false;

    for (line1, line2) in r1.zip(r2) {
        let l1 = line1.map_err(|_| "Error reading file1".to_string())?;
        let l2 = line2.map_err(|_| "Error reading file2".to_string())?;

        if l1 != l2 {
            differences_found = true;
            writeln!(diff, "FILE1: {}", l1).unwrap();
            writeln!(diff, "FILE2: {}", l2).unwrap();
            writeln!(diff, "-------------------------").unwrap();
        }
    }

    if !differences_found {
        writeln!(diff, "No differences found").unwrap();
    }

    Ok(())
}
