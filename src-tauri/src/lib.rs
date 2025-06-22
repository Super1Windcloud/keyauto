use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub mod execute;
pub mod fmt_display;
pub mod listen;
pub mod record;
pub use crate::execute::*;
pub use crate::listen::*;
#[allow(unused_imports)]
pub use crate::record::*;
pub mod register;
pub use crate::register::*;

#[tauri::command]
fn write_to_log_file(message: &str) -> Result<(), String> {
    let file = Path::new("../keyauto_log.txt");
    if file.exists() {
        let mut file = OpenOptions::new().append(true).open(file).unwrap();
        file.write_all(message.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    } else {
        let mut file = File::create(file).unwrap();
        file.write_all(message.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    Ok(())
}

#[tauri::command]
fn show_window(window: tauri::Window) -> Result<(), String> {
    if window.is_visible().unwrap() {
        return Ok(());
    }
    window.center().unwrap();
    window
        .show()
        .map_err(|e| format!("Failed to show window: {}", e))?;
    window
        .set_focus()
        .map_err(|e| format!("Failed to set focus: {}", e))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            write_to_log_file,
            show_window,
            init_record_key_file,
            execute_record_key_file,
            global_listen_key_down,
            global_listen_left_click,
            global_listen_right_click,
            global_stop_listen_key_down,
            read_record_key_from_file,
            output_run_task_count_config
        ])
        .setup(|app| {
            global_hotkeys_register(app);
            return Ok(());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
