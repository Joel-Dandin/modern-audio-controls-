use std::os::raw::{c_int, c_long};
use mpris::PlayerFinder;
use std::time::Duration;
use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD;

// Native D-Bus bindings for high performance
#[link(name = "dbus_media", kind = "static")]
extern "C" {
    fn media_init() -> c_int;
    fn media_cleanup();
    fn media_play() -> c_int;
    fn media_pause() -> c_int;
    fn media_next() -> c_int;
    fn media_previous() -> c_int;
    fn media_seek(offset_microseconds: c_long) -> c_int;
    fn media_get_position() -> c_long;
    fn media_set_position(position_microseconds: c_long) -> c_int;
}

// Fallback using mpris crate
fn find_player() -> Result<mpris::Player, Box<dyn std::error::Error>> {
    PlayerFinder::new()?.find_active().map_err(|e| e.into())
}

// Try native D-Bus first, fallback to mpris crate
pub fn init_media_system() -> Result<(), &'static str> {
    let ret = unsafe { media_init() };
    if ret == 0 {
        Ok(())
    } else {
        // Fallback initialization - just check if mpris works
        match find_player() {
            Ok(_) => Ok(()),
            Err(_) => Err("No media players available")
        }
    }
}

pub fn cleanup_media_system() {
    unsafe {
        media_cleanup();
    }
}

pub fn seek(offset: i64) {
    // Try native D-Bus first
    let offset_microseconds = offset * 1_000_000;
    let ret = unsafe { media_seek(offset_microseconds as c_long) };
    
    if ret != 0 {
        // Fallback to mpris
        if let Ok(player) = find_player() {
            let position = player.get_position().unwrap_or_default();
            let new_position_secs = position.as_secs_f64() + offset as f64;
            let new_position = Duration::from_secs_f64(new_position_secs.max(0.0));
            if let Some(track_id) = player.get_metadata().ok().and_then(|m| m.track_id()) {
                let _ = player.set_position(track_id, &new_position);
            }
        }
    }
}

pub fn get_media_state() -> Option<(f64, f64)> {
    // Try native D-Bus first
    let position_microseconds = unsafe { media_get_position() };
    
    if position_microseconds >= 0 {
        let position_seconds = position_microseconds as f64 / 1_000_000.0;
        // For native D-Bus, we'd need additional calls to get duration
        // For now, return position with estimated duration
        Some((position_seconds, 300.0))
    } else {
        // Fallback to mpris
        if let Ok(player) = find_player() {
            let position = player.get_position().unwrap_or_default().as_secs_f64();
            let duration = player.get_metadata().ok()
                .and_then(|m| m.length())
                .unwrap_or_default().as_secs_f64();
            Some((position, duration))
        } else {
            None
        }
    }
}

pub fn set_position(position: f64) {
    // Try native D-Bus first
    let position_microseconds = (position * 1_000_000.0) as c_long;
    let ret = unsafe { media_set_position(position_microseconds) };
    
    if ret != 0 {
        // Fallback to mpris
        if let Ok(player) = find_player() {
            let new_position = Duration::from_secs_f64(position.max(0.0));
            if let Some(track_id) = player.get_metadata().ok().and_then(|m| m.track_id()) {
                let _ = player.set_position(track_id, &new_position);
            }
        }
    }
}

pub fn next_track() {
    let ret = unsafe { media_next() };
    if ret != 0 {
        if let Ok(player) = find_player() {
            let _ = player.next();
        }
    }
}

pub fn previous_track() {
    let ret = unsafe { media_previous() };
    if ret != 0 {
        if let Ok(player) = find_player() {
            let _ = player.previous();
        }
    }
}

pub fn play() {
    let ret = unsafe { media_play() };
    if ret != 0 {
        if let Ok(player) = find_player() {
            let _ = player.play();
        }
    }
}

pub fn pause() {
    let ret = unsafe { media_pause() };
    if ret != 0 {
        if let Ok(player) = find_player() {
            let _ = player.pause();
        }
    }
}

pub fn get_media_info() -> Option<(String, Option<String>)> {
    // Use mpris for metadata as it's more reliable than our native D-Bus implementation
    if let Ok(player) = find_player() {
        if let Ok(metadata) = player.get_metadata() {
            let title = metadata.title().unwrap_or("Unknown Title").to_string();
            let art_url = metadata.art_url().map(|url| url.to_string());

            let base64_art = if let Some(url) = art_url {
                if url.starts_with("file://") {
                    let path = url.trim_start_matches("file://");
                    if let Ok(bytes) = std::fs::read(path) {
                        Some(format!("data:image/png;base64,{}", STANDARD.encode(bytes)))
                    } else {
                        None
                    }
                } else if url.starts_with("http://") || url.starts_with("https://") {
                    if let Ok(response) = reqwest::blocking::get(&url) {
                        if let Ok(bytes) = response.bytes() {
                            Some(format!("data:image/png;base64,{}", STANDARD.encode(bytes)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            Some((title, base64_art))
        } else {
            None
        }
    } else {
        None
    }
}
