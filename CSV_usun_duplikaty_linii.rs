use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let input = "kombinacje_4_z_5.csv";
    let output = "kombinacje_unique.csv";

    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut seen = HashSet::new();
    let mut out = File::create(output)?;

    for line in reader.lines() {
        let l = line?;

        // jeśli linia jeszcze nie była widziana → zapisujemy
        if seen.insert(l.clone()) {
            writeln!(out, "{}", l)?;
        }
    }

    Ok(())
}
