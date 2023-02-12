use macroquad::prelude::*;

#[macroquad::main("rusty-chip8")]
async fn main() {
    request_new_screen_size(64.0, 64.0);
    loop {}
}
