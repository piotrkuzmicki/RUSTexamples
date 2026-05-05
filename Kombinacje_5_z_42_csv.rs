use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("kombinacje_5_z_42.csv")?;
    let mut writer = BufWriter::new(file);

    // Wszystkie kombinacje 5 liczb z zakresu 1..=42
    for a in 1..=42 {
        for b in (a + 1)..=42 {
            for c in (b + 1)..=42 {
                for d in (c + 1)..=42 {
                    for e in (d + 1)..=42 {
                        writeln!(writer, "{},{},{},{},{}", a, b, c, d, e)?;
                    }
                }
            }
        }
    }

    Ok(())
}
