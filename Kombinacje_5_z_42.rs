use rust_xlsxwriter::{Workbook, Worksheet, Format, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let bold_format = Format::new().set_bold();

    // Nagłówki
    worksheet.write_string(0, 0, "A")?;
    worksheet.write_string(0, 1, "B")?;
    worksheet.write_string(0, 2, "C")?;
    worksheet.write_string(0, 3, "D")?;
    worksheet.write_string(0, 4, "E")?;

    let mut row: u32 = 1;

    // Wszystkie kombinacje 5 liczb z 1..=42
    for a in 1..=42 {
        for b in (a + 1)..=42 {
            for c in (b + 1)..=42 {
                for d in (c + 1)..=42 {
                    for e in (d + 1)..=42 {
                        worksheet.write_number(row, 0, a as f64)?;
                        worksheet.write_number(row, 1, b as f64)?;
                        worksheet.write_number(row, 2, c as f64)?;
                        worksheet.write_number(row, 3, d as f64)?;
                        worksheet.write_number(row, 4, e as f64)?;
                        row += 1;
                    }
                }
            }
        }
    }

    workbook.save("kombinacje_5_z_42.xlsx")?;
    Ok(())
}
