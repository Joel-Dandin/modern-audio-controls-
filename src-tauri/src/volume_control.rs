use std::os::raw::{c_long};

#[link(name = "volume", kind = "static")]
extern "C" {
    fn get_master_volume() -> c_long;
    fn set_master_volume(volume: c_long);
}

pub fn get_volume() -> i64 {
    unsafe {
        get_master_volume() as i64
    }
}

pub fn set_volume(volume: i64) {
    unsafe {
        set_master_volume(volume as c_long);
    }
}
