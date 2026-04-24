use enigo::{Enigo, KeyboardControllable, Key};
use std::{process::Command, thread, time::Duration};
// ---- 1. Uruchom Brave ----
fn main() {
    Command::new("C:/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe")
        .spawn()
        .expect("Nie można uruchomić Brave");


    // Czekamy aż Brave się otworzy
    thread::sleep(Duration::from_secs(2));

    let mut enigo = Enigo::new();

    // ---- 2. Fokus na pasek adresu (Ctrl+L) ----
    enigo.key_sequence_parse("{CTRL}");
    thread::sleep(Duration::from_millis(300));

    // ---- 3. Wpisz adres ----
    enigo.key_sequence("www.interia.pl");
    thread::sleep(Duration::from_millis(300));

    // ---- 4. Enter ----
    enigo.key_click(Key::Return);
}
