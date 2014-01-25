extern mod native;
extern mod sdl2;

use std::io::File;

mod sdl;
mod chip8;
mod timer;

#[start]
#[cfg(not(test))]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    let mut chip8 = chip8::Chip8::new();
    let mut file = File::open(&Path::new("pong.ch8"));
    // Load the file directly into the chip8 memory
    match file.read(chip8.mem.main) {
        Some(n) => println!("Loaded program of size: {}", n),
        None => fail!("Found empty file")
    }

    sdl::run(chip8);
}
