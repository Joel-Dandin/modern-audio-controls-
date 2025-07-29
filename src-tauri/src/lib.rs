use tauri::{AppHandle, Emitter};
use std::time::Duration;
use std::thread;

mod volume_control;
mod media_control;

#[tauri::command]
fn get_volume() -> i64 {
    volume_control::get_volume()
}

#[tauri::command]
fn set_volume(volume: i64) {
    volume_control::set_volume(volume)
}

#[tauri::command]
fn seek(offset: i64) {
    media_control::seek(offset);
}

#[tauri::command]
fn get_media_state() -> Option<(f64, f64)> {
    media_control::get_media_state()
}

#[tauri::command]
fn set_position(position: f64) {
    media_control::set_position(position);
}

fn spawn_volume_watcher(app: AppHandle) {
    thread::spawn(move || {
        let mut last_volume = volume_control::get_volume();
        loop {
            let current_volume = volume_control::get_volume();
            if current_volume != last_volume {
                last_volume = current_volume;
                app.emit("volume-changed", last_volume).unwrap();
            }
            thread::sleep(Duration::from_millis(200));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_volume, set_volume, seek, get_media_state, set_position])
        .setup(|app| {
            spawn_volume_watcher(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}