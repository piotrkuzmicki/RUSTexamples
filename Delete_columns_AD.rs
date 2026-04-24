use calamine::{open_workbook, Reader, Xlsx};
use rust_xlsxwriter::Workbook;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // ---- 1. Otwórz istniejący plik Excel ----
    let mut workbook_in: Xlsx<_> = open_workbook("plik.xlsx")?;
    let range = workbook_in
        .worksheet_range("Sheet1")
        .ok_or("Nie znaleziono arkusza")??;

    // ---- 2. Utwórz nowy plik Excel ----
    let mut workbook_out = Workbook::new();
    let worksheet = workbook_out.add_worksheet();

    // ---- 3. Skopiuj kolumny od E (index 4) do końca ----
    for (row_idx, row) in range.rows().enumerate() {
        for (col_idx, cell) in row.iter().enumerate().skip(4) {
            let new_col = (col_idx - 4) as u16; // przesunięcie kolumn

            if let Some(v) = cell.get_float() {
                worksheet.write_number(row_idx as u32, new_col, v)?;
            } else if let Some(s) = cell.get_string() {
                worksheet.write_string(row_idx as u32, new_col, s)?;
            }
        }
    }

    // ---- 4. Zapisz nowy plik ----
    workbook_out.save("file_no_A_D.xlsx")?;

    println!("Gotowe: kolumny A–D usunięte, zapisano output_no_A_D.xlsx");

    Ok(())
}
