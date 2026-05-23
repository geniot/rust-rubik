use std::ffi::CString;
use crate::bindings::{BeginDrawing, Color, ConfigFlags_FLAG_VSYNC_HINT, DrawText, EndDrawing, InitWindow, SetConfigFlags, WindowShouldClose};
use std::os::raw::c_char;
#[allow(warnings)]
mod bindings;

fn main() {
    println!("{}", std::env::consts::ARCH);
    let c_string = CString::new("Hello!").expect("CString::new failed");
    let name: *mut c_char = c_string.into_raw();
    let color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };

    unsafe {
        SetConfigFlags(ConfigFlags_FLAG_VSYNC_HINT);
        InitWindow(800, 600, name);
        while !WindowShouldClose() {
            BeginDrawing();
            DrawText(name, 10, 20, 20, color);
            EndDrawing();
        }
    }
}
