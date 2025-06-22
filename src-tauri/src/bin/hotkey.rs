 use rdev::{listen, Event, EventType};

 fn main() {
    fn handler(event: Event) {
        match event.event_type {
            EventType::KeyRelease(key) => {
                let name = event.name.unwrap_or_default();
                #[cfg(debug_assertions)]
                println!("Key {:?} released, event name: {}", key, name);
            }
            EventType::KeyPress(key) => {
                let name = event.name.unwrap_or_default();
                #[cfg(debug_assertions)]
                println!("Key {:?} pressed, event name: {}", key, name);
            }
            _ => {}
        }
    }

    if let Err(error) = listen(handler) {
        println!("Error: {:?}", error)
    }
}
