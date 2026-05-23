use crate::bindings::{BeginDrawing, ClearBackground, Color, ConfigFlags_FLAG_VSYNC_HINT, DrawText, EndDrawing, InitWindow, IsGamepadButtonPressed, SetConfigFlags, WindowShouldClose};
use std::ffi::CString;
use std::os::raw::c_char;
#[allow(warnings)]
mod bindings;

fn main() {
    println!("{}", std::env::consts::ARCH);
    let c_string = CString::new("Hello!").expect("CString::new failed");
    let name: *mut c_char = c_string.into_raw();
    let red = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    let white = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };

    unsafe {
        SetConfigFlags(ConfigFlags_FLAG_VSYNC_HINT);
        InitWindow(800, 600, name);
        let mut shouldExit = false;
        while !WindowShouldClose() && !shouldExit {
            BeginDrawing();
            ClearBackground(white);
            DrawText(name, 10, 20, 20, red);
            EndDrawing();
            if IsGamepadButtonPressed(0, 14) {
               shouldExit = true
            }
        }
    }
}
