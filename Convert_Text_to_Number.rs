use calamine::{open_workbook, Reader, Xlsx};
use rust_xlsxwriter::Workbook;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // ---- 1. Otwórz istniejący plik Excel ----
    let mut workbook_in: Xlsx<_> = open_workbook("plik1.xlsx")?;
    let range = workbook_in
        .worksheet_range("Sheet1")
        .ok_or("Nie znaleziono arkusza")??;

    // ---- 2. Utwórz nowy plik Excel ----
    let mut workbook_out = Workbook::new();
    let worksheet = workbook_out.add_worksheet();

    // ---- 3. Przetwarzaj każdy wiersz ----
    for (row_idx, row) in range.rows().enumerate() {
        for col_idx in 0..5 { // kolumny A–E
            let cell = row.get(col_idx);

            // ---- 4. Konwersja tekst → liczba ----
            if let Some(c) = cell {
                if let Some(v) = c.get_float() {
                    // liczba już jest liczbą
                    worksheet.write_number(row_idx as u32, col_idx as u16, v)?;
                } else if let Some(s) = c.get_string() {
                    // spróbuj sparsować tekst jako liczbę
                    if let Ok(parsed) = s.replace(",", ".").parse::<f64>() {
                        worksheet.write_number(row_idx as u32, col_idx as u16, parsed)?;
                    } else {
                        // nie da się sparsować → zostaw tekst
                        worksheet.write_string(row_idx as u32, col_idx as u16, s)?;
                    }
                }
            }
        }
    }

    // ---- 5. Zapisz nowy plik ----
    workbook_out.save("plik2.xlsx")?;

    println!("Gotowe: tekst przekonwertowany na liczby → output_numbers.xlsx");

    Ok(())
}
