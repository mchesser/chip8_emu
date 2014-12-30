use std;

pub use chip8::decoder::decode;
pub use chip8::cpu::Cpu;
pub use chip8::mem::Memory;
pub use chip8::input::Input;
pub use chip8::video::Video;

mod cpu;
mod mem;
mod input;
pub mod video;
mod decoder;

/// The timer speed = 60hz
pub const TICK_RATE: f64 = 1.0 / 60.0;
/// Clock rate of CPU = 1Mhz
pub const CLOCK_RATE: f64 = 1.0 / 1000.0;

pub struct Emulator {
    pub cpu: Cpu,
    pub mem: Memory,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: Cpu::new(),
            mem: Memory::new(),
        }
    }

    /// Execute the next frame
    pub fn frame(&mut self) {
        self.cpu.exec(&mut self.mem);
    }

    /// Return the internal video data
    pub fn display(&self) -> &[u8] {
        &self.mem.video.data
    }

    /// Signal a clock tick to the emulator
    pub fn tick(&mut self) {
        self.cpu.tick();
    }

    /// Signal a keydown event to the emulator
    pub fn keydown(&mut self, key_code: u8) {
        self.mem.input.set_keydown(key_code);
    }

    /// Signal a keyup event to the timer
    pub fn keyup(&mut self, key_code: u8) {
        self.mem.input.set_keyup(key_code);
    }

    pub fn poll_screen(&mut self) -> bool {
        std::mem::replace(&mut self.mem.video.screen_modified, false)
    }
}
