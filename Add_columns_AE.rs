use calamine::{open_workbook, Reader, Xlsx};
use rust_xlsxwriter::Workbook;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // ---- 1. Otwórz istniejący plik Excel ----
    let mut workbook_in: Xlsx<_> = open_workbook("plik2.xlsx")?;
    let range = workbook_in
        .worksheet_range("Sheet1")
        .ok_or("Nie znaleziono arkusza")??;

    // ---- 2. Utwórz nowy plik Excel ----
    let mut workbook_out = Workbook::new();
    let worksheet = workbook_out.add_worksheet();

    // ---- 3. Przetwarzaj każdy wiersz ----
    for (row_idx, row) in range.rows().enumerate() {
        let mut sum = 0.0;

        // Kolumny A–E → indeksy 0..4
        for col_idx in 0..5 {
            if let Some(cell) = row.get(col_idx) {
                if let Some(v) = cell.get_float() {
                    sum += v;
                }
            }
        }

        // ---- 4. Przepisz oryginalne dane A–E ----
        for (col_idx, cell) in row.iter().enumerate().take(5) {
            if let Some(v) = cell.get_float() {
                worksheet.write_number(row_idx as u32, col_idx as u16, v)?;
            } else if let Some(s) = cell.get_string() {
                worksheet.write_string(row_idx as u32, col_idx as u16, s)?;
            }
        }

        // ---- 5. Zapisz sumę do kolumny F (index 5) ----
        worksheet.write_number(row_idx as u32, 5, sum)?;
    }

    // ---- 6. Zapisz nowy plik ----
    workbook_out.save("plik3.xlsx")?;

    println!("Gotowe: suma A–E zapisana w kolumnie F → output_sum.xlsx");

    Ok(())
}
