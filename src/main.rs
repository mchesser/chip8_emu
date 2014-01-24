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
    let prog = File::open(&Path::new("test.ch8")).read_to_end();

    chip8.load_program(prog);
    sdl::run(chip8);
}
