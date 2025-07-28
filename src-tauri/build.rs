fn main() {
    cc::Build::new()
        .file("c/volume.c")
        .compile("libvolume.a");

    println!("cargo:rustc-link-lib=asound");

    tauri_build::build()
}
