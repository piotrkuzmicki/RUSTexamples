use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let input = "wyniki.csv";
    let output = "wyniki2.csv";

    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut out = File::create(output)?;

    for line in reader.lines() {
        let line = line?;
        let replaced = line.replace(";", ",");
        writeln!(out, "{}", replaced)?;
    }

    Ok(())
}

