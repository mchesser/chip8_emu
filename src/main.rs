#![feature(core, old_io, env, old_path)]
extern crate sdl2;
extern crate rand;

use std::old_io::File;

mod chip8;
mod timer;
mod client;

fn main() {
    let filename = std::env::args().skip(1).next().unwrap();
    let mut file = match File::open(&Path::new(filename)) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open input program: {}", e)
    };

    let mut emulator = chip8::Emulator::new();
    match file.read(&mut emulator.mem.ram) {
        Ok(n) => println!("Loaded program of size: {}", n),
        Err(e) => panic!("Failed to read file: {}", e)
    }

    if let Err(e) = client::run(emulator) {
        panic!("Client experienced a fatal error and had to close: {}", e);
    };
}
