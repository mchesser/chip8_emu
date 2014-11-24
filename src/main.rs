#![feature(if_let)]
#![feature(globs)]

extern crate sdl2;

use std::os;
use std::io::File;

mod chip8;
mod timer;
mod client;

fn main() {
    let args = os::args();
    let mut file = match File::open(&Path::new(args[1].as_slice())) {
        Ok(f) => f,
        Err(err) => panic!("Failed to open input program: {}", err)
    };

    let mut emulator = chip8::Emulator::new();
    match file.read(&mut emulator.mem.ram) {
        Ok(n) => println!("Loaded program of size: {}", n),
        Err(err) => panic!("Failed to read file: {}", err)
    }

    client::run(emulator);
}
