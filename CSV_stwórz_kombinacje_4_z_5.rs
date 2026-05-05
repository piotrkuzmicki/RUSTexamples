use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let input = "wyniki2.csv";
    let output = "kombinacje_4_z_5.csv";

    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut out = File::create(output)?;

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<u32> = line
            .split(',')
            .filter_map(|x| x.trim().parse().ok())
            .collect();

        // interesują nas tylko linie z dokładnie 5 liczbami
        if nums.len() != 5 {
            continue;
        }

        // generujemy wszystkie kombinacje 4 z 5
        for i in 0..5 {
            let mut combo = Vec::new();

            for j in 0..5 {
                if j != i {
                    combo.push(nums[j]);
                }
            }

            writeln!(
                out,
                "{},{},{},{}",
                combo[0], combo[1], combo[2], combo[3]
            )?;
        }
    }

    Ok(())
}

