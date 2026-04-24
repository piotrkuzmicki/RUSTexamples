use calamine::{open_workbook, Reader, Xlsx};
use rust_xlsxwriter::Workbook;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // ---- 1. Otwórz istniejący plik Excel ----
    let mut workbook_in: Xlsx<_> = open_workbook("plik4.xlsx")?;
    let range = workbook_in
        .worksheet_range("Sheet1")
        .ok_or("Nie znaleziono arkusza")??;

    // ---- 2. Utwórz nowy plik Excel ----
    let mut workbook_out = Workbook::new();
    let worksheet = workbook_out.add_worksheet();

    // ---- 3. Dodaj nagłówki w pierwszym wierszu ----
    worksheet.write_string(0, 0, "Number1")?;
    worksheet.write_string(0, 1, "Number2")?;
    worksheet.write_string(0, 2, "Number3")?;
    worksheet.write_string(0, 3, "Number4")?;
    worksheet.write_string(0, 4, "Number5")?;
    worksheet.write_string(0, 5, "Sum")?;
    worksheet.write_string(0, 6, "Average")?;

    // ---- 4. Przepisz dane z oryginalnego pliku, przesuwając je o 1 wiersz w dół ----
    for (row_idx, row) in range.rows().enumerate() {
        let new_row = (row_idx + 1) as u32; // przesunięcie o 1

        for (col_idx, cell) in row.iter().enumerate() {
            if let Some(v) = cell.get_float() {
                worksheet.write_number(new_row, col_idx as u16, v)?;
            } else if let Some(s) = cell.get_string() {
                worksheet.write_string(new_row, col_idx as u16, s)?;
            }
        }
    }

    // ---- 5. Zapisz nowy plik ----
    workbook_out.save("plik5.xlsx")?;

    println!("Gotowe: dodano pierwszy wiersz z nagłówkami → output_with_header.xlsx");

    Ok(())
}
