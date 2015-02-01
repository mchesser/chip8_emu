#![feature(core, rand, io, os, path)]
extern crate sdl2;

use std::os;
use std::old_io::File;

mod chip8;
mod timer;
mod client;

fn main() {
    let args = os::args();
    let mut file = match File::open(&Path::new(args[1].as_slice())) {
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
