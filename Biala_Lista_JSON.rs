use reqwest;
use serde_json::Value;
use std::fs;

#[tokio::main]
async fn main() {
    let nip = "1130089950";
    let date = "2026-04-26";
    let url = format!("https://wl-api.mf.gov.pl/api/search/nip/{}?date={}", nip, date);

    println!("Zapytanie do: {}", url);

    let response = reqwest::get(&url).await.unwrap();
    let body: Value = response.json().await.unwrap();

    let filename = format!("nip_{}_{}.json", nip, date);
    let pretty = serde_json::to_string_pretty(&body).unwrap();

    fs::write(&filename, &pretty).unwrap();

    println!("Zapisano do pliku: {}", filename);
    println!("{}", pretty);
}