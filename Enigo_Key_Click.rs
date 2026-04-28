use enigo::{Enigo, KeyboardControllable, Key, MouseControllable, MouseButton};
use std::{process::Command, thread, time::Duration};



fn main() {
    // 1. Uruchom Brave
    Command::new("C:/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe")
        .arg("https://agro-jacwing.pl/kontakt/") // <-- otwórz od razu stronę
        .spawn()
        .expect("Nie można uruchomić Brave");

    // 2. Daj Brave czas na start i załadowanie strony
    thread::sleep(Duration::from_secs(3));

    let mut enigo = Enigo::new();

    // 3. Wyślij 9× Tab
    for _ in 0..9 {
        enigo.key_click(Key::Tab);
        thread::sleep(Duration::from_millis(10));
    }

    // 4. Kliknij Enter (np. aktywuj link / pole)
    enigo.key_click(Key::Return);
    thread::sleep(Duration::from_millis(300));

    // 5. Wpisz tekst
    let text = "Piotr Ku";
    for ch in text.chars() {
        enigo.key_sequence(&ch.to_string());
        thread::sleep(Duration::from_millis(10));
    }
	
	// 3. Wyślij 1× Tab
        enigo.key_click(Key::Tab);
        thread::sleep(Duration::from_millis(10));
		
	// 5. Wpisz tekst
    let text = "piotr.kuzmicki@gmail.com";
    for ch in text.chars() {
        enigo.key_sequence(&ch.to_string());
        thread::sleep(Duration::from_millis(10));
    }
		enigo.key_click(Key::Tab);
        thread::sleep(Duration::from_millis(10));
	
	// 5. Wpisz tekst
    let text = "Informacje";
    for ch in text.chars() {
        enigo.key_sequence(&ch.to_string());
        thread::sleep(Duration::from_millis(10));
    }
		enigo.key_click(Key::Tab);
        thread::sleep(Duration::from_millis(10));
		
	// 5. Wpisz tekst
    let text = "Przykładowy tekst";
    for ch in text.chars() {
        enigo.key_sequence(&ch.to_string());
        thread::sleep(Duration::from_millis(10));
    }	
		enigo.key_click(Key::Tab);
        thread::sleep(Duration::from_millis(10));
	
	// 5. Wpisz tekst
    let text = "ABCD";
    for ch in text.chars() {
        enigo.key_sequence(&ch.to_string());
        thread::sleep(Duration::from_millis(10));
    }
		enigo.key_click(Key::Tab);
        thread::sleep(Duration::from_millis(10));
		enigo.key_click(Key::Return);
        thread::sleep(Duration::from_millis(10));
		enigo.key_click(Key::Tab);
        thread::sleep(Duration::from_millis(10));
		enigo.key_click(Key::Return);
        thread::sleep(Duration::from_millis(100));
		
		enigo.mouse_move_to(636, 54);
		enigo.mouse_click(MouseButton::Left);
		enigo.key_click(Key::Delete);
		thread::sleep(Duration::from_millis(3000));
		
	let text = "https://agro-jacwing.pl/atrakcje/";
    for ch in text.chars() {
        enigo.key_sequence(&ch.to_string());
        thread::sleep(Duration::from_millis(10));
	}
	enigo.key_click(Key::Return);
		


    // ---- Klik w punkt (71, 300) ----
    enigo.mouse_move_to(71, 300);
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(Duration::from_millis(300));

    // ---- Klik w punkt (71, 300) ----
    enigo.mouse_move_to(71, 300);
    enigo.mouse_click(MouseButton::Left);
		
	// Hold LEFT SHIFT
    enigo.key_down(Key::Shift);
    thread::sleep(Duration::from_millis(50));

	// Press RIGHT ARROW
	for _ in 0..9 {
    enigo.key_click(Key::DownArrow);
	}
	thread::sleep(Duration::from_millis(50));

    // Release LEFT SHIFT
    enigo.key_up(Key::Shift);
		
		
		
}
