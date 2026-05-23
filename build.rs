#![feature(str_as_str)]

use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-arg=-lX11");
    println!(
        "{}{}",
        "cargo:rustc-link-lib=static=raylib_",
        String::from(env::consts::ARCH)
    );
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("lib").display()
    );
}
