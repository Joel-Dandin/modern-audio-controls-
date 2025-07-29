use std::os::raw::{c_int, c_long};
use std::sync::Mutex;

#[link(name = "audio_native", kind = "static")]
extern "C" {
    fn audio_init(ctrl: *mut audio_control_t) -> c_int;
    fn audio_cleanup(ctrl: *mut audio_control_t);
    fn audio_get_volume(ctrl: *mut audio_control_t) -> c_long;
    fn audio_set_volume(ctrl: *mut audio_control_t, volume: c_long) -> c_int;
    fn audio_get_mute(ctrl: *mut audio_control_t) -> c_int;
    fn audio_set_mute(ctrl: *mut audio_control_t, mute: c_int) -> c_int;
}

#[repr(C)]
struct audio_control_t {
    backend: c_int,
    handle: *mut std::ffi::c_void,
    fd: c_int,
}

// SAFETY: We ensure thread safety through Mutex locking
unsafe impl Send for audio_control_t {}
unsafe impl Sync for audio_control_t {}

// Use Mutex for thread-safe access to audio control
static AUDIO_CTRL: Mutex<audio_control_t> = Mutex::new(audio_control_t {
    backend: 0,
    handle: std::ptr::null_mut(),
    fd: 0,
});

pub fn init_audio_system() -> Result<(), &'static str> {
    let mut ctrl = AUDIO_CTRL.lock().map_err(|_| "Failed to lock audio control")?;
    let ret = unsafe {
        audio_init(&mut *ctrl)
    };
    if ret == 0 {
        Ok(())
    } else {
        Err("Failed to initialize audio system")
    }
}

pub fn cleanup_audio_system() {
    if let Ok(mut ctrl) = AUDIO_CTRL.lock() {
        unsafe {
            audio_cleanup(&mut *ctrl);
        }
    }
}

pub fn get_volume() -> Result<i64, &'static str> {
    let mut ctrl = AUDIO_CTRL.lock().map_err(|_| "Failed to lock audio control")?;
    let volume = unsafe {
        audio_get_volume(&mut *ctrl)
    };
    if volume >= 0 {
        Ok(volume as i64)
    } else {
        Err("Failed to get volume")
    }
}

pub fn set_volume(volume: i64) -> Result<(), &'static str> {
    let mut ctrl = AUDIO_CTRL.lock().map_err(|_| "Failed to lock audio control")?;
    let ret = unsafe {
        audio_set_volume(&mut *ctrl, volume as c_long)
    };
    if ret == 0 {
        Ok(())
    } else {
        Err("Failed to set volume")
    }
}

// Mute functionality
pub fn get_mute() -> Result<bool, &'static str> {
    let mut ctrl = AUDIO_CTRL.lock().map_err(|_| "Failed to lock audio control")?;
    let mute = unsafe {
        audio_get_mute(&mut *ctrl)
    };
    Ok(mute != 0)
}

pub fn set_mute(mute: bool) -> Result<(), &'static str> {
    let mut ctrl = AUDIO_CTRL.lock().map_err(|_| "Failed to lock audio control")?;
    let ret = unsafe {
        audio_set_mute(&mut *ctrl, if mute { 1 } else { 0 })
    };
    if ret == 0 {
        Ok(())
    } else {
        Err("Failed to set mute")
    }
}
