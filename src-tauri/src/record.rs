use crate::{execute_key_press_event, execute_key_release_event, execute_left_button_event, execute_right_button_event, execute_wheel_event, IS_EXECUTING};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

pub static SHORTCUT_STOP: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RdevEventType {
    ButtonLeft((u32, u32)),
    ButtonRight((u32, u32)),
    KeyRelease(String),
    KeyPress(String),
    MouseWheel((i32, i32)),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RdevEvent {
    pub event_type: RdevEventType,
    pub event_name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RdevRecordEvent {
    #[serde(default)] // field not exist in json, default None
    pub(crate) run_task_count: Option<u32>,

    pub(crate) events: Vec<RdevEvent>,
}

pub fn save_event_backend(event: RdevEvent) -> Result<(), String> {
    let origin_content = read_record_key_from_file().unwrap_or_default();
    let record_event: RdevRecordEvent = serde_json::from_str(&origin_content).unwrap_or_default();
    #[cfg(debug_assertions)]
    println!("{:?}", record_event.events);

    let mut events = if record_event.events.iter().len() == 0 {
        vec![]
    } else {
        record_event.events
    };

    events.push(event);
    let json_obj = json!({
        "events" : events
    });
    let content = serde_json::to_string_pretty(&json_obj).map_err(|e| e.to_string())?;

    write_record_key_to_file(content.as_str())?;

    Ok(())
}

#[tauri::command]
pub fn init_record_key_file() -> Result<(), String> {
    let tempdir = std::env::temp_dir();
    let file_path = tempdir.join("keyauto_record.json");
    let mut file = File::create(file_path).unwrap();
    file.write_all(b"").unwrap();

    Ok(())
}

pub fn write_record_key_to_file(key_or_mouse_record: &str) -> Result<(), String> {
    let tempdir = std::env::temp_dir();
    let file_path = tempdir.join("keyauto_record.json");

    let mut file = File::create(file_path).unwrap();

    file.write_all(key_or_mouse_record.as_bytes()).unwrap();
    Ok(())
}

#[tauri::command]
pub fn read_record_key_from_file() -> Result<String, String> {
    let tempdir = std::env::temp_dir();
    let file_path = tempdir.join("keyauto_record.json");

    let content = std::fs::read_to_string(file_path).unwrap_or_default();

    Ok(content)
}

#[tauri::command]
pub fn execute_record_key_file(count: u32, stop: bool) -> Result<String, String> {
    let key_stop_flag = *SHORTCUT_STOP.lock().unwrap();
    let content = read_record_key_from_file()?;
    if content.is_empty() || stop || key_stop_flag {
        *IS_EXECUTING.lock().unwrap() = false;
        return Ok(content);
    }
    let events: RdevRecordEvent = serde_json::from_str(&content).unwrap();

    let events = events.events;
    for _ in 0..count {
        if stop || key_stop_flag {
            break;
        }
        events.iter().for_each(|event| {
            match event.event_type.clone() {
                RdevEventType::ButtonLeft((x, y)) => {
                    #[cfg(debug_assertions)]
                    println!("left button ({}, {})", x, y);
                    execute_left_button_event(x, y);
                    sleep(Duration::from_millis(500));
                }
                RdevEventType::ButtonRight((x, y)) => {
                    #[cfg(debug_assertions)]
                    println!("right button ({}, {})", x, y);
                    execute_right_button_event(x, y);
                    sleep(Duration::from_millis(500));
                }
                RdevEventType::KeyRelease(key) => {
                    #[cfg(debug_assertions)]
                    println!("key press {}", key);
                    execute_key_release_event(key);
                    // sleep(Duration::from_millis(300));
                }
                RdevEventType::MouseWheel((x, y)) => {
                    #[cfg(debug_assertions)]
                    println!("mouse wheel ({}, {})", x, y);
                    execute_wheel_event(x, y);
                    sleep(Duration::from_millis(100));
                }
                RdevEventType::KeyPress(key) => {
                    #[cfg(debug_assertions)]
                    println!("key released {}", key);
                    execute_key_press_event(key, event.event_name.clone());
                }
            }
        });
    }

    *IS_EXECUTING.lock().unwrap() = false; 
    Ok(content)
}

mod test_record_json {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_execute_task() {
        use super::execute_record_key_file;
        execute_record_key_file(1, false).unwrap();
    }
}
