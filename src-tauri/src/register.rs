
use tauri_plugin_global_shortcut::{Builder as GsBuilder, Shortcut, Modifiers, Code, ShortcutEvent};


pub fn global_hotkey_register() {


    GsBuilder::new()
        .on_event(|app, shortcut, event: ShortcutEvent| {
            if let ShortcutEvent::Pressed = event {
                // 将事件广播给 JS 前端
                let _ = app.emit_all("global-shortcut", shortcut.to_string());
            }
        })
        .build()
}