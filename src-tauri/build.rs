fn main() {
    // Compile audio native library
    cc::Build::new()
        .file("src/native/audio_native.c")
        .include("src/native")
        .compile("audio_native");
    
    // Compile D-Bus media library
    cc::Build::new()
        .file("src/native/dbus_media.c")
        .include("src/native")
        .include("/usr/include/dbus-1.0")
        .include("/usr/lib/x86_64-linux-gnu/dbus-1.0/include")
        .compile("dbus_media");
    
    // Link system libraries
    println!("cargo:rustc-link-lib=asound");
    println!("cargo:rustc-link-lib=dbus-1");
    
    tauri_build::build()
}
