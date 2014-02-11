pub static WIDTH: uint = 64;
pub static REPWIDTH: uint = WIDTH / 8;
pub static HEIGHT: uint = 32;

/// CHIP-8 gliphs, see: mattmik.com/chip8.html
pub static GLYPHS: [u8,..16*5] =  [
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
    repr: [u8,..REPWIDTH*HEIGHT]
}

impl Video {
    pub fn new() -> Video {
        Video { repr: [0x00, ..REPWIDTH*HEIGHT] }
    }

    pub fn clear(&mut self) {
        for px in self.repr.mut_iter() {
            *px = 0x0;
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, val: u8) -> u8 {
        if (x as uint) >= WIDTH || (y as uint) >= HEIGHT {
            return 0x1;
        }

        let i = x / 8 + y * REPWIDTH as u8;
        let shift = x % 8;

        if shift != 0 {
            let lval = val >> shift;
            let rval = val << (8 - shift);

            let lold = self.repr[i];
            self.repr[i] ^= lval;
            let rold = self.repr[i+1];
            self.repr[i+1] ^= rval;

            if flipped(lold, self.repr[i]) || flipped(rold, self.repr[i+1]) { 0x0 } else { 0x1 }
        }
        else {
            let old = self.repr[i];
            self.repr[i] ^= val;

            if flipped(old, self.repr[i]) { 0x0 } else { 0x1 }
        }
    }
}

/// Returns true if any of the bits have been fliped from set to unset
fn flipped(v1: u8, v2: u8) -> bool {
    v1 & !v2 != 0x0
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
