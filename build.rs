use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    if env::var("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER").is_ok()
        && env::var("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER").unwrap().contains("aarch64")
    {
        println!("cargo:rustc-link-lib=dylib=raylib");
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir).join("lib/arm64").display()
        );
    } else {
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=static=raylib");
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir).join("lib/amd64").display()
        );
    }


}
