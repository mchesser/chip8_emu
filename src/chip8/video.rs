pub const WIDTH: u8 = 64;
pub const HEIGHT: u8 = 32;
pub const BYTES_WIDTH: u8 = WIDTH / 8;

/// CHIP-8 gliphs, see: mattmik.com/chip8.html
pub static GLYPHS: [u8; 16*5] =  [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Video {
    pub data: [u8; BYTES_WIDTH as usize * HEIGHT as usize],
    pub screen_modified: bool,
}

impl Video {
    pub fn new() -> Video {
        Video {
            data: [0x00; BYTES_WIDTH as usize * HEIGHT as usize],
            screen_modified: true,
        }
    }

    pub fn clear(&mut self) {
        self.screen_modified = true;
        for px in self.data.iter_mut() {
            *px = 0x0;
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, val: u8) -> u8 {
        self.screen_modified = true;

        let x = x % WIDTH;
        let y = y % HEIGHT;

        let i = (x / 8 + y * BYTES_WIDTH) as usize;
        let shift = x % 8;

        // This draw command was not byte aligned, so we need xor over 2 bytes
        if shift != 0 {
            let i2 = ((x / 8 + 1) % BYTES_WIDTH + y * BYTES_WIDTH) as usize;

            let lval = val >> shift as usize;
            let rval = val << (8 - shift as usize);

            let lold = self.data[i];
            self.data[i] ^= lval;
            let rold = self.data[i2];
            self.data[i2] ^= rval;

            // If any bits were flipped as a result of drawing the sprite then return 1
            if flipped(lold, self.data[i]) || flipped(rold, self.data[i2]) { 1 } else { 0 }
        }
        else {
            let old = self.data[i];
            self.data[i] ^= val;

            // If any bits were flipped as a result of drawing the sprite then return 1
            if flipped(old, self.data[i]) { 1 } else { 0 }
        }
    }
}

/// Returns true if any of the bits have been fliped from set to unset
fn flipped(v1: u8, v2: u8) -> bool {
    v1 & !v2 != 0
}

#[test]
fn test_flipped() {
    assert!(flipped(0b_0000_0000, 0b_0001_0000) == false);
    assert!(flipped(0b_0001_0000, 0b_0000_0000) == true);
    assert!(flipped(0b_1010_0101, 0b_1111_1111) == false);
    assert!(flipped(0b_1010_0101, 0b_0000_0000) == true);
    assert!(flipped(0b_1010_0101, 0b_1010_0100) == true);
    assert!(flipped(0b_1111_0000, 0b_1111_1111) == false);
    assert!(flipped(0b_1111_0000, 0b_0000_1111) == true);
}
