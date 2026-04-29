use eframe::egui;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSV Folder Merge",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    dropped_folder: Option<PathBuf>,
    show_error: bool,
    error_message: String,
    show_success: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            dropped_folder: None,
            show_error: false,
            error_message: String::new(),
            show_success: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- DRAG & DROP FOLDER ---
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files = ctx.input(|i| i.raw.dropped_files.clone());
            if let Some(file) = files.first() {
                if let Some(path) = &file.path {
                    if path.is_dir() {
                        self.dropped_folder = Some(path.clone());
                    } else {
                        self.show_error = true;
                        self.error_message = "Drop a folder, not a file".to_string();
                    }
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drag & Drop Folder Containing CSV Files");
            ui.separator();

            if let Some(folder) = &self.dropped_folder {
                ui.label(format!("Folder: {}", folder.display()));

                if ui.button("MERGE CSV").clicked() {
                    match merge_csv_folder(folder) {
                        Ok(_) => {
                            self.show_success = true;
                        }
                        Err(err) => {
                            self.show_error = true;
                            self.error_message = err;
                        }
                    }
                }
            } else {
                ui.label("Przeciągnij folder tutaj");
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
                        ui.label("CSV files merged successfully!");
                        if ui.button("OK").clicked() {
                            self.show_success = false;
                        }
                    });
            }
        });
    }
}

// ------------------------------------------------------------
// MERGE CSV WITH COLUMN VALIDATION
// ------------------------------------------------------------
fn merge_csv_folder(folder: &Path) -> Result<(), String> {
    let mut files: Vec<_> = fs::read_dir(folder)
        .map_err(|_| "Cannot read folder".to_string())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("csv"))
        .collect();

    if files.is_empty() {
        return Err("Folder does not contain CSV files".to_string());
    }

    files.sort();

    let output_path = folder.join("merged_output.csv");
    let mut output = File::create(&output_path)
        .map_err(|_| "Cannot create output file".to_string())?;

    let mut expected_cols: Option<usize> = None;
    let mut first_file = true;

    for file_path in files {
        let file = File::open(&file_path)
            .map_err(|_| format!("Cannot open file: {}", file_path.display()))?;
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            let line = line.map_err(|_| "Error reading CSV".to_string())?;

            let cols = line.split(',').count();

            // Validate column count
            if expected_cols.is_none() {
                expected_cols = Some(cols);
            } else if expected_cols.unwrap() != cols {
                return Err(format!(
                    "Column count mismatch in file: {}\nExpected {}, got {}",
                    file_path.display(),
                    expected_cols.unwrap(),
                    cols
                ));
            }

            // Skip header in next files
            if i == 0 && !first_file {
                continue;
            }

            writeln!(output, "{}", line)
                .map_err(|_| "Cannot write to output file".to_string())?;
        }

        first_file = false;
    }

    Ok(())
}
