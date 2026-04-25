use reqwest;
use serde_json::Value;
use std::fs;

#[tokio::main]
async fn main() {
    let krs = "0000053665"; // zmień na dowolny numer KRS

    let url = format!(
        "https://api-krs.ms.gov.pl/api/krs/OdpisAktualny/{}?rejestr=P&format=json",
        krs
    );

    println!("Zapytanie do: {}", url);

    let response = reqwest::get(&url).await.unwrap();
    let body: Value = response.json().await.unwrap();

    let filename = format!("krs_{}.json", krs);
    let pretty = serde_json::to_string_pretty(&body).unwrap();

    fs::write(&filename, &pretty).unwrap();

    println!("Zapisano do pliku: {}", filename);
    println!("{}", pretty);
}