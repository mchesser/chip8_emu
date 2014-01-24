use chip8::mem;
use std::rand;
use std::rand::Rng;

/// CHIP-8 Registers
struct Registers {
    // Data registers
    V: [u8, ..16],
    // Address register
    I:  u16
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            V: [0, ..16],
            I: 0,
        }
    }

    /// Add x to register
    pub fn addx(&mut self, r: u8, x: u8) {
        let val = self.V[r as uint] as u32 + x as u32;
        // VF = 1 if there was a carry otherwise VF = 0
        self.V[0xF] = if val & 0x100 != 0 { 1 } else { 0 };
        self.V[r as uint] = val as u8;
    }

    /// Sub register by x
    pub fn subx(&mut self, r: u8, x: u8) {
        let val = self.V[r as uint ] as u32 - x as u32;
        // VF = 0 if there was a borrow otherwise VF = 1
        self.V[0xF] = if val & 0x100 == 0 { 1 } else { 0 };
        self.V[r as uint] = val as u8;
    }

    /// Add the value of one register to another
    pub fn addr(&mut self, r1: u8, r2: u8) {
        self.addx(r1, self.V[r2 as uint]);
    }

    /// Set reg1 = reg1 - reg2
    pub fn subr(&mut self, r1: u8, r2: u8) {
        self.subx(r1, self.V[r2 as uint]);
    }

    /// Set reg1 = reg2 - reg1
    pub fn sub2r(&mut self, r1: u8, r2: u8) {
        let x = self.V[r1 as uint];
        self.V[r1 as uint] = self.V[r2 as uint];
        self.subx(r1, x);
    }

    /// Sets register to r1 | r2
    pub fn orr(&mut self, r1: u8, r2: u8) {
        self.V[r1 as uint] = self.V[r1 as uint] | self.V[r2 as uint];
    }

    /// Sets register to r1 & r2
    pub fn andr(&mut self, r1: u8, r2: u8) {
        self.V[r1 as uint] = self.V[r1 as uint] & self.V[r2 as uint];
    }

    /// Sets register to r1 ^ r2
    pub fn xorr(&mut self, r1: u8, r2: u8) {
        self.V[r1 as uint] = self.V[r1 as uint] ^ self.V[r2 as uint];
    }

    /// Left shift by 1
    pub fn lshf(&mut self, r1: u8) {
        self.V[0xF] = self.V[r1 as uint] & 0x1;
        self.V[r1 as uint] = self.V[r1 as uint] >> 1;
    }

    /// Right shift by 1
    pub fn rshf(&mut self, r1: u8) {
        let mut sig_bit = 0;
        for i in range(0, 8).invert() {
            if self.V[r1 as uint] >> i & 0x1 == 0x1 {
                sig_bit = i as u8;
                break;
            }
        }
        self.V[0xF] = 0x1 << sig_bit;
        self.V[r1 as uint] = self.V[r1 as uint] << 1;
    }

    /// If register is equal to value skip next op
    pub fn cmp_val(&mut self, r1: u8, val: u8) -> bool {
        self.V[r1] == val
    }

    /// If registers are equal skip next op
    pub fn cmp_reg(&mut self, r1: u8, r2: u8) -> bool {
        self.V[r1] == self.V[r2]
    }

    /// If register is not equal to next value skip next op
    pub fn cmp_nval(&mut self, r1: u8, val: u8) -> bool {
        self.V[r1] != val
    }

    /// If registers are not equal skip next op
    pub fn cmp_nreg(&mut self, r1: u8, r2: u8) -> bool {
        self.V[r1] != self.V[r2]
    }

    /// Sets register to random number & x
    pub fn rnd_reg(&mut self, r1: u8, x: u8) {
        self.V[r1] = rand::task_rng().gen::<u8>() & x;
    }
}

/*impl fmt::Default for Registers {
    fn fmt(r: &Registers, f: &mut fmt::Formatter) {
        write!(f.buf, "V0:{:2x} V1:{:2x} V2:{:2x} V3:{:2x} V4:{:2x} /
                       V5:{:2x} V6:{:2x} V7:{:2x} V8:{:2x} V9:{:2x} /
                       VA:{:2x} VB:{:2x} VC:{:2x} VD:{:2x} VE:{:2x} /
                       VF:{:2x} I:{:4x}",
            r.V[0x0], r.V[0x1], r.V[0x2], r.V[0x3], r.V[0x4], r.V[0x5],
            r.V[0x6], r.V[0x7], r.V[0x8], r.V[0x9], r.V[0xA], r.V[0xB],
            r.V[0xC], r.V[0xD], r.V[0xE], r.V[0xF], r.I);
    }
}*/

/// CHIP-8 Timers
struct Timers {
    delay: u8,
    sound: u8
}

impl Timers {
    fn new() -> Timers {
        Timers {
            delay: 0,
            sound: 0
        }
    }
}

pub struct CPU {
    r: Registers,
    t: Timers,
    pc: u16
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            r: Registers::new(),
            t: Timers::new(),
            pc: mem::MAIN
        }
    }

    pub fn exec(&mut self, mem: &mut mem::Memory) {
        let op = mem.rw(self.pc);
        //println!("0x{:04x}", op);
        self.pc += 2;

        // See http://en.wikipedia.org/wiki/CHIP-8#Opcode_table for opcode table
        match mask01(op) {
            0x0 => {
                match op {
                    0x00E0 => mem.clear_disp(),
                    0x00EE => self.ret(mem),
                    // No idea what to do with this opcode
                    _      => {}
                }
            },
            0x1 => self.pc = mask13(op),
            0x2 => self.call(mem, mask13(op)),

            // Comparisons
            0x3 => if self.r.cmp_val(mask11(op), mask22(op)) { self.pc += 2 },
            0x4 => if self.r.cmp_nval(mask11(op), mask22(op)){ self.pc += 2 },
            0x5 => if self.r.cmp_reg(mask11(op), mask21(op)) { self.pc += 2 },

            // Arithmetic
            0x6 => self.r.V[mask11(op)] = mask22(op),
            0x7 => self.r.addx(mask11(op), mask22(op)),
            0x8 => {
                let r1 = mask11(op);
                let r2 = mask21(op);
                match mask31(op) {
                    0x0 => self.r.V[r2] = self.r.V[r2],
                    0x1 => self.r.orr(r1, r2),
                    0x2 => self.r.andr(r1, r2),
                    0x3 => self.r.xorr(r1, r2),
                    0x4 => self.r.addr(r1, r2),
                    0x5 => self.r.subr(r1, r2),
                    0x6 => self.r.rshf(r1),
                    0x7 => self.r.sub2r(r1, r2),
                    0xE => self.r.lshf(r1),
                    _   => fail!("Invalid opcode")
                }
            },

            0x9 => if self.r.cmp_nreg(mask11(op), mask21(op)) { self.pc += 2 },

            0xA => self.r.I = mask13(op),
            0xB => self.pc = mask13(op) + self.r.V[0] as u16,

            0xC => self.r.rnd_reg(mask11(op), mask22(op)),
            0xD => {
                self.r.V[0xF] = mem.draw(self.r.V[mask11(op)], self.r.V[mask21(op)], mask31(op), self.r.I);
            },
            0xE => {
                match mask22(op) {
                    0x9E => if mem.input.keydown(mask11(op)) { self.pc += 2 },
                    0xA1 => if !mem.input.keydown(mask11(op)) { self.pc += 2 },
                    _    => fail!("Invalid opcode")
                }
            },

            0xF => {
                match mask22(op) {
                    0x07 => self.r.V[mask11(op)] = self.t.delay,
                    0x0A => {
                        match mem.input.get_key() {
                            Some(key) => self.r.V[mask11(op)] = key,
                            None      => self.pc -= 2
                        }
                    },
                    0x15 => self.t.delay = self.r.V[mask11(op)],
                    0x18 => self.t.sound = self.r.V[mask11(op)],
                    0x1E => self.r.I += self.r.V[mask11(op)] as u16,
                    0x29 => self.r.I = mem.glyph_addr(self.r.V[mask11(op)]),
                    0x33 => { // ???
                        let val = self.r.V[mask11(op)];
                        mem.wb(self.r.I, (val / 100) % 10);
                        mem.wb(self.r.I, (val / 10) % 10);
                        mem.wb(self.r.I, (val / 1) % 10);
                    },
                    0x55 => {
                        for i in range(0, mask11(op)) {
                            mem.wb(self.r.I + i as u16, self.r.V[i as u16]);
                        }
                    },
                    0x65 => {
                        for i in range(0, mask11(op)) {
                            self.r.V[i as u16] = mem.rb(self.r.I + i as u16);
                        }
                    },
                    _    => fail!("Invalid opcode")
                }
            },


            _ => fail!("Unreachable code")
        }
    }

    /// Ticks the timer one step
    pub fn tick(&mut self) {
        if self.t.delay < 0 {
            self.t.delay -= 1;
        }
        if self.t.sound < 0 {
            self.t.sound -= 1;
        }
    }

    /// Make a subroutine call
    fn call(&mut self, mem: &mut mem::Memory, addr: u16) {
        mem.stack.push(self.pc);
        self.pc = addr;
    }

    /// Return from a subroutine call
    fn ret(&mut self, mem: &mut mem::Memory) {
        self.pc = mem.stack.pop();
    }
}

///
/// Bit select patterns
///

fn mask01(op: u16) -> u8 { ((op & 0xF000) >> 12) as u8 }
fn mask11(op: u16) -> u8 { ((op & 0x0F00) >> 8) as u8 }
fn mask13(op: u16) -> u16 { op & 0x0FFF }
fn mask21(op: u16) -> u8 { ((op & 0x00F0) >> 4) as u8 }
fn mask22(op: u16) -> u8 { (op & 0x00FF) as u8 }
fn mask31(op: u16) -> u8 { (op & 0x000F) as u8 }

#[cfg(test)]
mod tests {
    // TODO: add many more tests
    use super::Registers;

    #[test]
    fn test_add() {
        let mut r = Registers::new();
        r.addx(0x0, 5);
        assert_eq!(r.V[0x0], 5);

        r.addx(0x1, 5);
        assert_eq!(r.V[0x1], 5);

        r.addr(0x0, 0x1);
        assert_eq!(r.V[0x0], 10);
    }

    #[test]
    fn test_add_carry() {
        let mut r = Registers::new();
        r.addx(0x0, 0xFF);
        assert_eq!(r.V[0x0], 0xFF);
        assert_eq!(r.V[0xF], 0x00);

        r.addx(0x0, 1);
        assert_eq!(r.V[0x0], 0x00);
        assert_eq!(r.V[0xF], 0x01);
    }

    #[test]
    fn test_sub() {
        let mut r = Registers::new();
        r.V[0x0] = 10;
        r.subx(0x0, 5);
        assert_eq!(r.V[0x0], 5);

        r.V[0x1] = 10;
        r.subx(0x1, 5);
        assert_eq!(r.V[0x1], 5);

        r.subr(0x0, 0x1);
        assert_eq!(r.V[0x0], 0);
    }

    #[test]
    fn test_sub_borrow() {
        let mut r = Registers::new();
        r.subx(0x0, 0x01);
        assert_eq!(r.V[0x0], 0xFF);
        assert_eq!(r.V[0xF], 0x00);

        r.subx(0x0, 0xFF);
        assert_eq!(r.V[0x0], 0x00);
        assert_eq!(r.V[0xF], 0x01);
    }

}
