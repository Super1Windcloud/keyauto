use rdev::{Button, Event, EventType, Key};
use std::io::Write;
use std::process;
use sysinfo::{Pid, ProcessesToUpdate, System};

const LISTENER_EXE: &[u8] = include_bytes!("../assits/keyauto_listener.exe");

fn is_process_running(process_name: &str) -> bool {
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::All, true);

    for (_pid, process) in system.processes() {
        if process.name().to_str().unwrap().to_lowercase() == process_name.to_lowercase() {
            return true;
        }
    }
    false
}

fn get_process_id(process_name: &str) -> Option<u32> {
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::All, true);

    for (pid, process) in system.processes() {
        if process.name().to_str().unwrap().to_lowercase() == process_name.to_lowercase() {
            return Some(pid.as_u32());
        }
    }
    None
}

#[tauri::command]
pub fn global_listen_key_down() -> Result<u32, String> {
    let temp_bin = std::env::temp_dir().join("keyauto_listener.exe");
    if !temp_bin.exists()   {
        let mut file = std::fs::File::create(&temp_bin).unwrap();
        file.write_all(LISTENER_EXE).unwrap();
        file.flush().unwrap();
    }
    if is_process_running("keyauto_listener.exe") {
        println!("Key auto listener is already running");
        let pid = get_process_id("keyauto_listener.exe").unwrap();
        return Ok(pid);
    }

    let child = process::Command::new(temp_bin)
        .spawn()
        .expect("Failed to execute child");

    let pid = child.id();
    println!("Listening on process PID: {}", pid);
    Ok(pid)
}

#[tauri::command]
pub fn global_stop_listen_key_down(pid: u32) -> Result<bool, String> {
    println!("Stop listening on process PID: {}", pid);
    let mut system = System::new();
    system.refresh_all();

    if let Some(process) = system.process(Pid::from(pid as usize)) {
        let result = process.kill();
        Ok(result)
    } else {
        Ok(false)
    }
}

#[allow(unused)]
fn key_down_event_handler(event: Event) {
    match event.event_type {
        EventType::KeyRelease(key) => {
            println!("Key Down: {:?}", key);
            if key == Key::KeyQ {
                println!("Exiting");
                process::exit(0);
            }
        }
        EventType::ButtonPress(button) => {
            match button {
                Button::Left => {
                    println!("Left Click")
                    // let  position = event.position;
                    // println!("Left Click at ({}, {})", position.x, position.y);
                }
                Button::Right => println!("Right Click"),
                _ => {}
            }
        }
        EventType::MouseMove { x, y } => println!("Mouse Move: ({}, {})", x, y),
        EventType::Wheel { delta_x, delta_y } => {
            println!("Mouse Wheel: ({}, {})", delta_x, delta_y)
        }
        _ => {}
    }
}

#[tauri::command]
pub fn global_listen_left_click() {}

#[tauri::command]
pub fn global_listen_right_click() {}

mod test_listen {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_key() {
        global_listen_key_down().unwrap();
    }
}
