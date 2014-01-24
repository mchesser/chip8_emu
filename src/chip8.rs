use chip8::cpu::CPU;
use chip8::mem::Memory;
use timer::Timer;

mod cpu;
pub mod mem;
pub mod video;
pub mod input;

pub struct Chip8 {
    cpu: CPU,
    mem: Memory,
    timer: Timer
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: CPU::new(),
            mem: Memory::new(),
            timer: Timer::new()
        }
    }
    
    pub fn load_program(&mut self, prog: &[u8]) {
        self.mem.load(prog, mem::MAIN);
    }
    
    /// Executes the next frame
    pub fn frame(&mut self) {
        // Tick timers at 60Hz
        if (self.timer.elapsed() >= 60) {
            self.cpu.tick();
            self.timer.reset();
            self.cpu.exec(&mut self.mem);
        }
    }
    
    /// Returns the video data
    pub fn image<'a>(&'a self) -> &'a [u8] {
        self.mem.video.repr.as_slice()
    }
    
    /// Keydown event
    pub fn keydown(&mut self, key_code: u8) {
        self.mem.input.set_keydown(key_code);
    }
    
    /// Keyup event
    pub fn keyup(&mut self, key_code: u8) {
        self.mem.input.set_keyup(key_code);
    }
}