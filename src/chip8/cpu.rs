use rand::{self, Rng};
use std::iter::range_inclusive;
use chip8::{self, mem};

pub use self::Operation::*;
pub use self::Value::*;

pub const OPCODE_SIZE: u16 = 2;

pub type RegId = u8;
#[derive(Debug)]
pub enum Value {
    Reg(RegId),
    Const(u8),
}

#[derive(Debug)]
pub enum Operation {
    // Special 1
    CallRCA(u16),
    Unimplemented(u16),

    // Control flow
    Jump(u16),
    Call(u16),
    Return,
    SkipIfEq(RegId, Value),
    SkipIfNotEq(RegId, Value),

    // Arithmetic
    Set(RegId, Value),
    Add(RegId, Value),
    Sub(RegId, RegId),
    SubRev(RegId, RegId),
    Or(RegId, RegId),
    And(RegId, RegId),
    Xor(RegId, RegId),
    Shr(RegId, RegId),
    Shl(RegId, RegId),

    // Address manipulation
    SetAddr(u16),
    AddAddr(RegId),
    JumpWithOffset(u16),

    // Manipulation on multiple bytes
    StoreBcd(RegId),
    LoadBytes(RegId),
    StoreBytes(RegId),

    // Special 2
    GetRandom(RegId, u8),
    Draw(u8, u8, u8),
    LoadGlyph(RegId),
    ClearScreen,

    // Keyboard management
    SkipIfKeyPressed(RegId),
    SkipIfKeyNotPressed(RegId),
    KeyWait(RegId),

    // Timer management
    GetDelay(RegId),
    SetDelay(RegId),
    SetSound(RegId),
}

#[allow(non_snake_case)]
pub struct Cpu {
    // Delay timer register
    delay: u8,

    // Sound timer register
    sound: u8,

    // General purpose data registers
    V: [u8; 16],

    // Address register
    I: u16,

    // Program counter
    pc: u16,

    // Cpu random number generator
    rng: rand::ThreadRng,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            delay: 0,
            sound: 0,
            V: [0; 16],
            I: 0,
            pc: mem::RAM_START,
            rng: rand::thread_rng(),
        }
    }

    pub fn tick(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
    }

    pub fn exec(&mut self, mem: &mut mem::Memory) {
        let op = chip8::decode(mem.read_word(self.pc));
        self.pc += OPCODE_SIZE;

        match op {
            //
            // Special 1
            //
            CallRCA(..) => unimplemented!(),
            Unimplemented(code) => println!("Unimplemented opcode: {}", code),

            //
            // Control flow
            //
            Jump(addr) => self.pc = addr,

            Call(addr) => {
                mem.stack_push(self.pc);
                self.pc = addr;
            },

            Return => self.pc = mem.stack_pop(),

            SkipIfEq(r, Const(val)) => {
                if self.V[r as usize] == val {
                    self.pc += 2;
                }
            },
            SkipIfEq(r1, Reg(r2)) => {
                if self.V[r1 as usize] == self.V[r2 as usize] {
                    self.pc += 2;
                }
            },

            SkipIfNotEq(reg, Const(val)) => {
                if self.V[reg as usize] != val {
                    self.pc += 2;
                }
            },
            SkipIfNotEq(r1, Reg(r2)) => {
                if self.V[r1 as usize] != self.V[r2 as usize] {
                    self.pc += 2;
                }
            },

            //
            // Arithmetic
            //
            Set(reg, Const(val)) => self.V[reg as usize] = val,
            Set(r1, Reg(r2)) => self.V[r1 as usize] = self.V[r2 as usize],

            Add(reg, Const(val)) => self.V[reg as usize] += val,
            Add(r1, Reg(r2)) => {
                let result = self.V[r1 as usize] as u16 + self.V[r2 as usize] as u16;
                self.V[0xF] = if overflow8(result) { 1 } else { 0 };
                self.V[r1 as usize] = result as u8;
            },

            Sub(r1, r2) => {
                let result = self.V[r1 as usize] as u16 - self.V[r2 as usize] as u16;
                self.V[0xF] = if overflow8(result) { 0 } else { 1 };
                self.V[r1 as usize] = result as u8;
            },
            SubRev(r1, r2) => {
                let result = self.V[r2 as usize] as u16 - self.V[r1 as usize] as u16;
                self.V[0xF] = if overflow8(result) { 0 } else { 1 };
                self.V[r1 as usize] = result as u8;
            },

            Or(r1, r2) => self.V[r1 as usize] = self.V[r1 as usize] | self.V[r2 as usize],
            And(r1, r2) => self.V[r1 as usize] = self.V[r1 as usize] & self.V[r2 as usize],
            Xor(r1, r2) => self.V[r1 as usize] = self.V[r1 as usize] ^ self.V[r2 as usize],

            Shr(r1, r2) => {
                self.V[0xF] = self.V[r2 as usize] & 0x1;
                self.V[r1 as usize] = self.V[r2 as usize] >> 1;
            },
            Shl(r1, r2) => {
                self.V[0xF] = self.V[r2 as usize] >> 7;
                self.V[r1 as usize] = self.V[r2 as usize] << 1;
            },

            //
            // Address manipulation
            //
            SetAddr(addr) => self.I = addr,
            AddAddr(r) => {
                let result = self.I + self.V[r as usize] as u16;
                self.V[0xF] = if overflow12(result) { 1 } else { 0 };
                self.I = result % 0x1000;
            },
            JumpWithOffset(addr) => self.pc = addr + self.V[0] as u16,

            //
            // Manipulation on multiple bytes
            //
            StoreBcd(r) => {
                let val = self.V[r as usize];
                mem.write_byte(self.I, (val / 100) % 10);
                mem.write_byte(self.I+1, (val / 10) % 10);
                mem.write_byte(self.I+2, (val / 1) % 10);
            },

            LoadBytes(r) => {
                for i in range_inclusive(0, r as usize) {
                    self.V[i] = mem.read_byte(self.I + i as u16);
                }
            },
            StoreBytes(r) => {
                for i in range_inclusive(0, r as usize) {
                    mem.write_byte(self.I + i as u16, self.V[i]);
                }
            },

            //
            // Special 2
            //
            GetRandom(r, val) => self.V[r as usize] = self.rng.gen::<u8>() & val,
            Draw(x, y, n) => {
                self.V[0xF] = mem.draw(self.V[x as usize], self.V[y as usize], n, self.I);
            },
            LoadGlyph(r) => self.I = mem.load_glyph(self.V[r as usize]),
            ClearScreen => mem.clear_disp(),

            //
            // Keyboard management
            //
            SkipIfKeyPressed(r) => {
                if mem.is_keydown(self.V[r as usize]) {
                    self.pc += 2;
                }
            },
            SkipIfKeyNotPressed(r) => {
                if !mem.is_keydown(self.V[r as usize]) {
                    self.pc += 2;
                }
            },
            KeyWait(r) => {
                match mem.get_key() {
                    Some(key) => self.V[r as usize] = key,
                    None => self.pc -= 2,
                }
            },

            //
            // Timer management
            //
            GetDelay(r) => self.V[r as usize] = self.delay,
            SetDelay(r) => self.delay = self.V[r as usize],
            SetSound(r) => self.sound = self.V[r as usize],
        }
    }
}

/// Check if a value is too large to store in a 8 bit number
fn overflow8(value: u16) -> bool {
    value > 0xFF
}

fn overflow12(value: u16) -> bool {
    value > 0xFFF
}
