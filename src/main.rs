use std::{fs, io::Read};

mod chip8;
mod client;

#[macroquad::main("CHIP8 Emulator")]
async fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let mut file = match fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open input program: {}", e),
    };

    let mut emulator = chip8::Emulator::new();
    match file.read(&mut emulator.mem.ram) {
        Ok(n) => println!("Loaded program of size: {}", n),
        Err(e) => panic!("Failed to read file: {}", e),
    }

    if let Err(e) = client::run(emulator).await {
        panic!("Client experienced a fatal error and had to close: {}", e);
    };
}
