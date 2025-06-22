use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use rdev::{listen, Event, Key};
use keyauto_lib::fmt_display::RdevKeyStruct;
use keyauto_lib::{save_event_backend, RdevEvent, RdevEventType};
use mouse_position::mouse_position::Mouse;
use rdev::{Button, EventType};

const SAVE: bool = true;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let handle = thread::spawn(move || {
        let result = listen(move |event| {
            if !r.load(Ordering::SeqCst) {
                println!("Received shutdown signal");
                return;
            }
            handle_event(event, &r);
        });

        if let Err(e) = result {
            println!("Listen error: {:?}", e);
        }
    });

    // 模拟等待用户按下 F4 并设置退出标志
    loop {
        if !running.load(Ordering::SeqCst) {
            break;
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("Waiting for listener to exit...");
    handle.join().unwrap();
    println!("Gracefully exited");
}

fn handle_event(event: Event, running: &Arc<AtomicBool>) {
    match event.event_type {
        EventType::ButtonPress(button) => {
            let position = Mouse::get_mouse_position();
            let (x, y) = match position {
                Mouse::Position { x, y } => (x as u32, y as u32),
                Mouse::Error => {
                    println!("Mouse error");
                    return;
                }
            };
            let event_type = match button {
                Button::Left => RdevEventType::ButtonLeft((x, y)),
                Button::Right => RdevEventType::ButtonRight((x, y)),
                _ => return,
            };
            let event = RdevEvent {
                event_type,
                event_name: match button {
                    Button::Left => "left_click".into(),
                    Button::Right => "right_click".into(),
                    _ => "".into(),
                },
            };
            if SAVE {
                save_event_backend(event).unwrap();
            }
        }
        EventType::KeyRelease(key) => {
            if key == Key::F4 {
                println!("F4 released, exiting...");
                running.store(false, Ordering::SeqCst);
                return;
            }
            let name = event.name.unwrap_or_default();
            let key_event = RdevEvent {
                event_type: RdevEventType::KeyRelease(RdevKeyStruct::from(key).to_string()),
                event_name: name,
            };
            if SAVE {
                save_event_backend(key_event).unwrap();
            }
        }
        EventType::KeyPress(key) => {
            let name = event.name.unwrap_or_default();
            let key_event = RdevEvent {
                event_type: RdevEventType::KeyPress(RdevKeyStruct::from(key).to_string()),
                event_name: name,
            };
            if SAVE {
                save_event_backend(key_event).unwrap();
            }
        }
        EventType::Wheel { delta_x, delta_y } => {
            let wheel_event = RdevEvent {
                event_type: RdevEventType::MouseWheel((delta_x as i32, delta_y as i32)),
                event_name: "mouse_wheel".into(),
            };
            if SAVE {
                save_event_backend(wheel_event).unwrap();
            }
        }
        _ => {}
    }
}
