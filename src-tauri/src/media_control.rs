
use mpris::PlayerFinder;
use std::time::Duration;

fn find_player() -> Result<mpris::Player, Box<dyn std::error::Error>> {
    PlayerFinder::new()?.find_active().map_err(|e| e.into())
}

pub fn seek(offset: i64) {
    if let Ok(player) = find_player() {
        let position = player.get_position().unwrap_or_default();
        let new_position_secs = position.as_secs_f64() + offset as f64;
        let new_position = Duration::from_secs_f64(new_position_secs.max(0.0));
        if let Some(track_id) = player.get_metadata().unwrap().track_id() {
            player.set_position(track_id, &new_position).ok();
        }
    }
}

pub fn get_media_state() -> Option<(f64, f64)> {
    if let Ok(player) = find_player() {
        let position = player.get_position().unwrap_or_default().as_secs_f64();
        let duration = player.get_metadata().unwrap().length().unwrap_or_default().as_secs_f64();
        Some((position, duration))
    } else {
        None
    }
}

pub fn set_position(position: f64) {
    if let Ok(player) = find_player() {
        let new_position = Duration::from_secs_f64(position.max(0.0));
        if let Some(track_id) = player.get_metadata().unwrap().track_id() {
            player.set_position(track_id, &new_position).ok();
        }
    }
}
