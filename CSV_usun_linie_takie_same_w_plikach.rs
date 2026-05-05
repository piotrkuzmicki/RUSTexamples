use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let file_komb = "kombinacje.csv";
    let file_wyniki = "wyniki2.csv";
    let output = "kombinacje_bez_wynikow.csv";

    // --- 1. Wczytaj wyniki2.csv do HashSet (szybkie wyszukiwanie) ---
    let f2 = File::open(file_wyniki)?;
    let r2 = BufReader::new(f2);

    let mut to_remove = HashSet::new();

    for line in r2.lines() {
        let l = line?;
        to_remove.insert(l);
    }

    // --- 2. Przetwarzaj kombinacje.csv linia po linii ---
    let f1 = File::open(file_komb)?;
    let r1 = BufReader::new(f1);

    let mut out = File::create(output)?;

    for line in r1.lines() {
        let l = line?;

        // jeśli linia NIE występuje w wyniki2.csv → zapisujemy
        if !to_remove.contains(&l) {
            writeln!(out, "{}", l)?;
        }
    }

    Ok(())
}

