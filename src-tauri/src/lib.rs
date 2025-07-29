use tauri::{AppHandle, Emitter};
use std::time::Duration;
use std::thread;

mod volume_control;
mod media_control;

// Initialize systems on startup
fn init_native_systems() -> Result<(), String> {
    volume_control::init_audio_system()
        .map_err(|e| format!("Audio init failed: {}", e))?;
    
    media_control::init_media_system()
        .map_err(|e| format!("Media init failed: {}", e))?;
    
    Ok(())
}

// Cleanup systems
fn cleanup_native_systems() {
    volume_control::cleanup_audio_system();
    media_control::cleanup_media_system();
}

#[tauri::command]
fn get_volume() -> i64 {
    volume_control::get_volume().unwrap_or(0)
}

#[tauri::command]
fn set_volume(volume: i64) {
    let _ = volume_control::set_volume(volume.clamp(0, 100));
}

#[tauri::command]
fn get_mute() -> bool {
    volume_control::get_mute().unwrap_or(false)
}

#[tauri::command]
fn set_mute(mute: bool) {
    let _ = volume_control::set_mute(mute);
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

#[tauri::command]
fn next_track() {
    media_control::next_track();
}

#[tauri::command]
fn previous_track() {
    media_control::previous_track();
}

#[tauri::command]
fn play() {
    media_control::play();
}

#[tauri::command]
fn pause() {
    media_control::pause();
}

#[tauri::command]
fn get_media_info() -> Option<(String, Option<String>)> {
    media_control::get_media_info()
}

fn spawn_volume_watcher(app: AppHandle) {
    thread::spawn(move || {
        let mut last_volume = volume_control::get_volume().unwrap_or(0);
        loop {
            let current_volume = volume_control::get_volume().unwrap_or(0);
            if current_volume != last_volume {
                last_volume = current_volume;
                let _ = app.emit("volume-changed", last_volume);
            }
            thread::sleep(Duration::from_millis(100)); // Faster polling for better responsiveness
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize native systems first
    if let Err(e) = init_native_systems() {
        eprintln!("Failed to initialize native systems: {}", e);
        std::process::exit(1);
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_volume, set_volume, get_mute, set_mute, seek, get_media_state, set_position, next_track, previous_track, play, pause, get_media_info])
        .setup(|app| {
            spawn_volume_watcher(app.handle().clone());
            Ok(())
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                cleanup_native_systems();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
