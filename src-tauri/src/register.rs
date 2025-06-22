use crate::{
    execute_record_key_file, global_listen_key_down, global_stop_listen_key_down,
    init_record_key_file, RdevRecordEvent, SHORTCUT_STOP,
};
use once_cell::sync::Lazy;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::thread;
use tauri::App;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};

static PID: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));
pub static IS_EXECUTING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub static IS_RECORDING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn global_hotkeys_register(app: &mut App) {
    #[cfg(desktop)]
    {
        let handle = app.handle();

        let start_record_shortcut = Shortcut::new(None, Code::F3);
        let stop_record_shortcut = Shortcut::new(None, Code::F4);
        let start_execute_shortcut = Shortcut::new(None, Code::F5);
        let stop_execute_shortcut = Shortcut::new(None, Code::F6);
        let clear_record_events_shortcut = Shortcut::new(None, Code::F9);

        handle
            .plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |_app, shortcut, event| {
                        if shortcut == &start_record_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    println!("start_record_shortcut Pressed!");
                                }
                                ShortcutState::Released => {
                                    let pid = PID.lock().unwrap();
                                    if *pid != 0 {
                                        return;
                                    }
                                    let pid = global_listen_key_down().unwrap();
                                    *PID.lock().unwrap() = pid;
                                    *IS_RECORDING.lock().unwrap() = true;
                                }
                            }
                        } else if shortcut == &stop_record_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    let pid = *PID.lock().unwrap();
                                    #[cfg(debug_assertions)]
                                    println!("stopping listen pid: {}", pid);
                                    if pid == 0 {
                                        return;
                                    }
                                    global_stop_listen_key_down(pid).unwrap();
                                    *PID.lock().unwrap() = 0;
                                }
                                ShortcutState::Released => {
                                    println!("stop_record_shortcut  Released!");
                                    let pid = *PID.lock().unwrap();
                                    if pid == 0 {
                                        return;
                                    }
                                    #[cfg(debug_assertions)]
                                    println!("stopping listen pid: {}", pid);
                                    global_stop_listen_key_down(pid).unwrap();
                                    *PID.lock().unwrap() = 0;
                                    *IS_RECORDING.lock().unwrap() = false;
                                }
                            }
                        } else if shortcut == &start_execute_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    println!("start_execute_shortcut  Pressed!");
                                }
                                ShortcutState::Released => {
                                    let execute_flag = *IS_EXECUTING.lock().unwrap();
                                    let is_recording = *IS_RECORDING.lock().unwrap();
                                    if !execute_flag && !is_recording {
                                        let temp_dir = std::env::temp_dir();
                                        let path = temp_dir.join("keyauto_record.json");
                                        let content = std::fs::read_to_string(path).unwrap_or_default();
                                        if content.is_empty() {
                                            return;
                                        }
                                        let obj: RdevRecordEvent =
                                            serde_json::from_str(&content).unwrap_or_default();
                                        let count = obj.run_task_count;
                                        if let Some(count) = count {
                                            thread::spawn(move || {
                                                execute_record_key_file(count, false).unwrap();
                                            });
                                        }
                                    }
                                    *IS_EXECUTING.lock().unwrap() = true;
                                }
                            }
                        } else if shortcut == &stop_execute_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    println!("stop_execute_shortcut   Pressed!");
                                }
                                ShortcutState::Released => {
                                    let key_stop_flag = *SHORTCUT_STOP.lock().unwrap();
                                    if key_stop_flag {
                                        return;
                                    }
                                    *SHORTCUT_STOP.lock().unwrap() = true;
                                }
                            }
                        } else if shortcut == &clear_record_events_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    println!("clear_record_events_shortcut Pressed!");
                                }
                                ShortcutState::Released => {
                                    init_record_key_file().unwrap();
                                }
                            }
                        }
                    })
                    .build(),
            )
            .expect("Failed to register global shortcut");

        let plugin_handle = handle.global_shortcut();
        
        plugin_handle.register(start_record_shortcut).unwrap();
        plugin_handle.register(stop_record_shortcut).unwrap();
        plugin_handle.register(start_execute_shortcut).unwrap();
        plugin_handle.register(stop_execute_shortcut).unwrap();
        plugin_handle
            .register(clear_record_events_shortcut)
            .unwrap();
    }
}

#[tauri::command]
pub fn output_run_task_count_config(run_task_count: u32) -> String {
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("keyauto_record.json");

    if !path.exists() {
        init_record_key_file().unwrap();
    }
    let content = std::fs::read_to_string(&path).unwrap();
    let obj: RdevRecordEvent = serde_json::from_str(&content).unwrap();

    let events = obj.events;

    let obj = RdevRecordEvent {
        run_task_count: Some(run_task_count),
        events,
    };
    let json = serde_json::to_string_pretty(&obj).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .unwrap();

    file.write_all(json.as_bytes()).unwrap();
    format!("任务数量：{}，文件路径：{}", run_task_count, path.display())
}

#[test]
fn test_output_config() {
    output_run_task_count_config(3);
}
