use chip8::video;
use chip8::video::Video;
use chip8::input::Input;

pub static MAIN: u16 = 0x200;
static END: u16 = 0xFFF;

static GLYPHS: u16 = MAIN - 5 * 0xF;
static DISPLAY: u16 = END - 256;

static MEM_SIZE: u16 = DISPLAY - MAIN;

pub struct Memory {
    priv main: [u8, ..MEM_SIZE],
    stack: Stack,
    video: Video,
    input: Input
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            main: [0x0, ..MEM_SIZE],
            stack: Stack::new(),
            video: Video::new(),
            input: Input::new()
        }
    }
    
    /// Load a program into memory at the specified address
    pub fn load(&mut self, prog: &[u8], addr: u16) {
        assert!(addr >= MAIN && addr < DISPLAY, "Invalid address");
        //assert!(addr + prog.len() as u16 < MEM_SIZE, "Insufficient memory");
        for i in range(0, prog.len() as u16) {
            self.wb(addr + i as u16, prog[i]);
        }
    }
    
    /// Reads a byte
    pub fn rb(&self, addr: u16) -> u8 {       
        *(self.map(addr))
    }
    
    /// Reads a word
    pub fn rw(&self, addr: u16) -> u16 {
        (self.rb(addr) as u16) << 8 | (self.rb(addr + 1) as u16)
    }
    
    /// Writes a byte
    pub fn wb(&mut self, addr: u16, val: u8) {       
        *(self.map_mut(addr)) = val;
    }
    
    /// Writes a word
    pub fn ww(&mut self, addr: u16, val: u16) {
        self.wb(addr, (val >> 8) as u8);
        self.wb(addr+1, val as u8);
    }
    
    /// Map to mutable address
    fn map_mut<'a>(&'a mut self, addr: u16) -> &'a mut u8 {
        assert!(addr < END, "Invalid address");
        
        if addr < GLYPHS {
            // Not sure what maps to this
            fail!("Reserved address")
        }
        else if addr < MAIN {
            // Glyphs are read_only
            fail!("Read only memory")
        }
        else if addr < DISPLAY {
            &'a mut self.main[addr - MAIN]
        }
        else {
            &'a mut self.video.repr[addr - DISPLAY]
        }
    }
    
    /// Map to immutable address
    fn map<'a>(&'a self, addr: u16) -> &'a u8 {
        assert!(addr < END, "Invalid address");
        
        if addr < GLYPHS {
            // Not sure what maps to this
            fail!("Reserved address")
        }
        else if addr < MAIN {
            &'a video::GLYPHS[addr - GLYPHS]
        }
        else if addr < DISPLAY {
            &'a self.main[addr - MAIN]
        }
        else {
            &'a self.video.repr[addr - DISPLAY]
        }
    }
    
    
    ///
    /// Graphics operations
    ///
    
    /// Get the address
    pub fn glyph_addr(&self, val: u8) -> u16 {
        assert!(val <= 0xF, "Invalid glyph");
        GLYPHS + val as u16 * 8
    }
    
    /// Clear the display
    pub fn clear_disp(&mut self) {
        self.video.clear();
    }
    
    /// Draws a sprite at addr on the screen at x, y
    /// Returns 1 if any screen pixels are flipped from set to unset when the sprite is drawn, 
    /// or 0 if that doesn't happen.
    pub fn draw(&mut self, x: u8, y: u8, h: u8, addr: u16) -> u8 {
        let mut flipped = 0x0;
        for dy in range(0, h) {
            let glyph = *(self.map(addr + dy as u16));
            flipped |= self.video.draw(x, y + dy, glyph);
        }
        flipped
    }
}

// Stack size supports 16 levels of nesting
static STACK_SIZE: uint = 16;

struct Stack {
    priv values: [u16, ..STACK_SIZE],
    priv i: uint
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            values: [0, ..STACK_SIZE],
            i: 0
        }
    }
    
    pub fn push(&mut self, val: u16) {
        // assert!(i < STACK_SIZE, "Stack overflow");

        self.i += 1;
        self.values[self.i] = val;
    }
    
    pub fn pop(&mut self) -> u16 {
        // assert!(i != 0, "Stack failure");

        let val = self.values[self.i];
        self.i -= 1;
        val
    }
}

#[cfg(test)]
mod tests {
    use super::{Memory, MAIN};
    
    #[test]
    fn test_read_write_byte() {
        let mut mem = Memory::new();
        assert_eq!(mem.rb(MAIN as u16), 0x00);
        mem.wb(MAIN as u16, 0xFF);
        assert_eq!(mem.rb(MAIN as u16), 0xFF);
    }
    
    #[test]
    fn test_read_write_word() {
        let mut mem = Memory::new();
        assert_eq!(mem.rw(MAIN as u16), 0x0000);
        mem.ww(MAIN as u16, 0xFFFF);
        assert_eq!(mem.rw(MAIN as u16), 0xFFFF);
    }
}