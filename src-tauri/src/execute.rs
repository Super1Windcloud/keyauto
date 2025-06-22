#![allow(unused_imports)]
use crate::fmt_display::RdevKeyStruct;
use enigo::Coordinate::Abs;
use enigo::Direction::Press;
use enigo::{
    Axis, Button,
    Direction::{Click, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
pub use std::thread::sleep;
use std::time::Duration;

static ENIGO: Lazy<Mutex<Enigo>> =
    Lazy::new(|| Mutex::new(Enigo::new(&Settings::default()).unwrap()));

pub fn execute_left_button_event(x: u32, y: u32) {
    let mut enigo = ENIGO.lock().unwrap();
    enigo.move_mouse(x as i32, y as i32, Abs).unwrap();
    enigo.button(Button::Left, Click).unwrap();
}

pub fn execute_right_button_event(x: u32, y: u32) {
    let mut enigo = ENIGO.lock().unwrap();
    enigo.move_mouse(x as i32, y as i32, Abs).unwrap();
    enigo.button(Button::Right, Click).unwrap();
}

pub fn execute_key_press_event(key: String, event_name: String) {
    if event_name.is_empty() {
        // ! System Operation
        execute_system_operation_key_event(key);
        sleep(Duration::from_millis(300));
    } else {
        // ! Text Input
        execute_text_input_key_event(key, event_name);
    }
}
pub fn execute_key_release_event(key: String) {
    let mut enigo = ENIGO.lock().unwrap();
    let key: RdevKeyStruct = key.parse().unwrap();
    let mock_key = key.to_enigo_key();
    if mock_key.is_none() {
        return;
    }
    let mock_key = mock_key.unwrap();
    enigo.key(mock_key, Release).unwrap();
}

fn execute_text_input_key_event(_: String, event_name: String) {
    let mut enigo = ENIGO.lock().unwrap();
    enigo.text(event_name.as_str()).unwrap()
}

fn execute_system_operation_key_event(key: String) {
    let mut enigo = ENIGO.lock().unwrap();
    let key: RdevKeyStruct = key.parse().unwrap();
    let mock_key = key.to_enigo_key();
    if mock_key.is_none() {
        return;
    }
    let mock_key = mock_key.unwrap();
    enigo.key(mock_key, Press).unwrap();
}

pub fn execute_wheel_event(deltax: i32, deltay: i32) {
    let mut enigo = ENIGO.lock().unwrap();

    if deltax == 1 {
        enigo.scroll(1, Axis::Horizontal).unwrap()
    } else if deltax == -1 {
        enigo.scroll(-1, Axis::Horizontal).unwrap()
    } else if deltay == 1 {
        enigo.scroll(1, Axis::Vertical).unwrap()
    } else if deltay == -1 {
        enigo.scroll(-1, Axis::Vertical).unwrap()
    }
}

#[test]
#[ignore]
fn test() {
    let now = std::time::Instant::now();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.key(Key::Meta, Press).unwrap();
    enigo.key(Key::Meta, Release).unwrap();
    sleep(Duration::from_millis(300));
    enigo
        .text("Hello World! here is a lot of text  ❤️")
        .unwrap();
    println!("{:?}", now.elapsed());
}

#[test]
fn test_record_quote_and_meta() {
    let mut enigo = ENIGO.lock().unwrap();
    enigo.key(Key::Meta, Press).unwrap();
    enigo.key(Key::Meta, Release).unwrap();
    sleep(Duration::from_millis(300));
    enigo.key(Key::OEMJump, Press).unwrap();
    enigo.key(Key::OEMJump, Release).unwrap();
    sleep(Duration::from_millis(300));
    enigo.key(Key::OEMPlus, Press).unwrap();
    enigo.key(Key::OEMPlus, Release).unwrap();
    enigo.key(Key::OEM102, Press).unwrap();
    enigo.key(Key::OEM102, Release).unwrap();
    enigo.key(Key::OEM1, Press).unwrap();
    enigo.key(Key::OEM1, Release).unwrap();
    enigo.key(Key::OEM2, Press).unwrap();
    enigo.key(Key::OEM2, Release).unwrap();
    enigo.key(Key::OEM3, Press).unwrap();
    enigo.key(Key::OEM3, Release).unwrap();
    enigo.key(Key::OEMNECEqual, Press).unwrap();
    enigo.key(Key::OEMNECEqual, Release).unwrap();
}

#[test]
fn test_mouse_move() {
    execute_left_button_event(1000, 1000)
}

#[test]
fn test_key_release_for_input_text() {
    execute_key_press_event("KeyA".to_string(), "a".to_string());
    execute_key_press_event("KeyA".to_string(), "a".to_string());
    execute_key_press_event("KeyA".to_string(), "a".to_string());
    execute_key_press_event("KeyA".to_string(), "a".to_string());
    execute_key_release_event("KeyA".to_string());
}
