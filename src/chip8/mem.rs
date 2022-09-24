use crate::chip8;

pub const GLYPHS_START: u16 = 0x000;
pub const RAM_START: u16 = 0x200;
pub const RESERVED_START: u16 = 0xEA0;
pub const DISPLAY_START: u16 = 0xF00;
pub const TOTAL_MEMORY: u16 = 0x1000;

pub const RAM_SIZE: u16 = RESERVED_START - RAM_START;
pub const STACK_SIZE: usize = 16;

pub static ZERO: u8 = 0;

pub struct Memory {
    pub ram: [u8; (RAM_SIZE as usize)],
    stack: Vec<u16>,
    pub input: chip8::Input,
    pub video: chip8::Video,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; (RAM_SIZE as usize)],
            stack: vec![],
            input: chip8::Input::new(),
            video: chip8::Video::new(),
        }
    }

    pub fn stack_push(&mut self, addr: u16) {
        if self.stack.len() < STACK_SIZE {
            self.stack.push(addr);
        }
        else {
            panic!("Stack overflow");
        }
    }

    pub fn stack_pop(&mut self) -> u16 {
        match self.stack.pop() {
            Some(val) => val,
            None => panic!("Stack underflow"),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        *(self.map_addr(addr))
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr) as u16) << 8 | (self.read_byte(addr + 1) as u16)
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        *(self.map_addr_mut(addr)) = val;
    }

    pub fn is_keydown(&mut self, key: u8) -> bool {
        self.input.is_keydown(key)
    }

    pub fn get_key(&mut self) -> Option<u8> {
        self.input.get_key()
    }

    pub fn clear_disp(&mut self) {
        self.video.clear();
    }

    /// Draws a sprite at addr on the screen at x, y
    /// Returns 1 if any screen pixels are flipped from set to unset when the sprite is drawn,
    /// or 0 if that doesn't happen.
    pub fn draw(&mut self, x: u8, y: u8, h: u8, addr: u16) -> u8 {
        let mut flipped = 0x0;
        for dy in 0..h {
            let draw_data = *(self.map_addr(addr + dy as u16));
            flipped |= self.video.draw(x, y + dy, draw_data);
        }
        flipped
    }

    pub fn load_glyph(&self, val: u8) -> u16 {
        assert!(val <= 0xF, "Invalid glyph");
        GLYPHS_START + val as u16 * 5
    }

    fn map_addr(&self, addr: u16) -> &u8 {
        if addr >= TOTAL_MEMORY {
            panic!("Address too large: {}", addr);
        }
        else if addr >= DISPLAY_START {
            &self.video.data[(addr - DISPLAY_START) as usize]
        }
        else if addr >= RESERVED_START {
            panic!("Attempted to access reserved address: {}", addr);
        }
        else if addr >= RAM_START {
            &self.ram[(addr - RAM_START) as usize]
        }
        else if addr >= GLYPHS_START {
            if addr < chip8::video::GLYPHS.len() as u16 {
                &chip8::video::GLYPHS[(addr - GLYPHS_START) as usize]
            }
            else {
                // The glyphs don't use up the entire reserved space, so return 0 if the address is
                // larger than the number of glyphs
                &ZERO
            }
        }
        else {
            unreachable!()
        }
    }

    fn map_addr_mut(&mut self, addr: u16) -> &mut u8 {
        if addr >= TOTAL_MEMORY {
            panic!("Address too large: {}", addr);
        }
        else if addr >= DISPLAY_START {
            &mut self.video.data[(addr - DISPLAY_START) as usize]
        }
        else if addr >= RESERVED_START {
            panic!("Attempted to access reserved address: {}", addr);
        }
        else if addr >= RAM_START {
            &mut self.ram[(addr - RAM_START) as usize]
        }
        else if addr >= GLYPHS_START {
            panic!("Attempted to access read only memory: {}", addr);
        }
        else {
            unreachable!()
        }
    }
}
