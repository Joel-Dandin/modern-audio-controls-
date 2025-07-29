use alsa::mixer::{Mixer, SelemChannelId, SelemId};

const CARD_NAME: &str = "default";
const SELEM_NAME: &str = "Master";

pub fn get_volume() -> Result<i64, Box<dyn std::error::Error>> {
    let mixer = Mixer::new(CARD_NAME, false)?;
    let selem_id = SelemId::new(SELEM_NAME, 0);
    
    let selem = mixer
        .find_selem(&selem_id)
        .ok_or("Could not find Master volume control")?;
    
    let (min, max) = selem.get_playback_volume_range();
    let volume = selem.get_playback_volume(SelemChannelId::FrontLeft)?;
    
    let percentage = (100 * (volume - min)) / (max - min);
    Ok(percentage)
}

pub fn set_volume(volume: i64) -> Result<(), Box<dyn std::error::Error>> {
    let mixer = Mixer::new(CARD_NAME, false)?;
    let selem_id = SelemId::new(SELEM_NAME, 0);
    
    let selem = mixer
        .find_selem(&selem_id)
        .ok_or("Could not find Master volume control")?;
    
    let (min, max) = selem.get_playback_volume_range();
    let target_volume = min + (volume * (max - min)) / 100;
    
    selem.set_playback_volume_all(target_volume)?;
    Ok(())
}

// Compatibility functions that match the original API
pub fn get_volume_compat() -> i64 {
    get_volume().unwrap_or(0)
}

pub fn set_volume_compat(volume: i64) {
    let _ = set_volume(volume.clamp(0, 100));
}
