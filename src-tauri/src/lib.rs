mod volume_control;

#[tauri::command]
fn get_volume() -> i64 {
    volume_control::get_volume()
}

#[tauri::command]
fn set_volume(volume: i64) {
    volume_control::set_volume(volume)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_volume, set_volume])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
