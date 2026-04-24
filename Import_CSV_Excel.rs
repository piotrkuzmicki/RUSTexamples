use csv::ReaderBuilder;
use rust_xlsxwriter::{Workbook, Worksheet};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // ---- 1. Otwórz CSV z separatorem ; ----
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("plik.csv")?;

    // ---- 2. Utwórz nowy plik Excel ----
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // ---- 3. Przepisz wszystkie wiersze CSV do Excela ----
    for (row_idx, row) in reader.records().enumerate() {
        let record = row?;

        for (col_idx, cell) in record.iter().enumerate() {
            worksheet.write_string(row_idx as u32, col_idx as u16, cell)?;
        }
    }

    // ---- 4. Zapisz plik ----
    workbook.save("plik.xlsx")?;

    println!("Gotowe: CSV zaimportowany do output.xlsx");

    Ok(())
}
