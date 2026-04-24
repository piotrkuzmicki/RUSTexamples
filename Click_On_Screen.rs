use enigo::{Enigo, MouseControllable, MouseButton};
use std::{thread, time::Duration};

fn main() {
    let mut enigo = Enigo::new();

    // ---- Klik w punkt (200, 200) ----
    enigo.mouse_move_to(200, 200);
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(Duration::from_millis(300));

    // ---- Klik w punkt (400, 200) ----
    enigo.mouse_move_to(400, 200);
    enigo.mouse_click(MouseButton::Left);
}
