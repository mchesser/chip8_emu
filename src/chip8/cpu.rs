use std::rand;
use std::rand::Rng;
use std::iter::range_inclusive;
use chip8;
use chip8::mem;

pub use self::Operation::*;
pub use self::Value::*;

pub const OPCODE_SIZE: u16 = 2;

pub type RegId = u8;
#[deriving(Show)]
pub enum Value {
    Reg(RegId),
    Const(u8),
}

#[deriving(Show)]
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
            CallRCA(_) => unimplemented!(),
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

            SkipIfEq(reg, Const(val)) => {
                if self.V[reg as usize] == val {
                    self.pc += 2;
                }
            },
            SkipIfEq(reg1, Reg(reg2)) => {
                if self.V[reg1 as usize] == self.V[reg2 as usize] {
                    self.pc += 2;
                }
            },

            SkipIfNotEq(reg, Const(val)) => {
                if self.V[reg as usize] != val {
                    self.pc += 2;
                }
            },
            SkipIfNotEq(reg1, Reg(reg2)) => {
                if self.V[reg1 as usize] != self.V[reg2 as usize] {
                    self.pc += 2;
                }
            },

            //
            // Arithmetic
            //
            Set(reg, Const(val)) => self.V[reg as usize] = val,
            Set(reg1, Reg(reg2)) => self.V[reg1 as usize] = self.V[reg2 as usize],

            Add(reg, Const(val)) => self.V[reg as usize] += val,
            Add(reg1, Reg(reg2)) => {
                let result = self.V[reg1 as usize] as u16 + self.V[reg2 as usize] as u16;
                self.V[0xF] = if overflow8(result) { 1 } else { 0 };
                self.V[reg1 as usize] = result as u8;
            },

            Sub(reg1, reg2) => {
                let result = self.V[reg1 as usize] as u16 - self.V[reg2 as usize] as u16;
                self.V[0xF] = if overflow8(result) { 0 } else { 1 };
                self.V[reg1 as usize] = result as u8;
            },
            SubRev(reg1, reg2) => {
                let result = self.V[reg2 as usize] as u16 - self.V[reg1 as usize] as u16;
                self.V[0xF] = if overflow8(result) { 0 } else { 1 };
                self.V[reg1 as usize] = result as u8;
            },

            Or(reg1, reg2) => self.V[reg1 as usize] = self.V[reg1 as usize] | self.V[reg2 as usize],
            And(reg1, reg2) => self.V[reg1 as usize] = self.V[reg1 as usize] & self.V[reg2 as usize],
            Xor(reg1, reg2) => self.V[reg1 as usize] = self.V[reg1 as usize] ^ self.V[reg2 as usize],

            Shr(reg1, reg2) => {
                self.V[0xF] = self.V[reg2 as usize] & 0x1;
                self.V[reg1 as usize] = self.V[reg2 as usize] >> 1;
            },
            Shl(reg1, reg2) => {
                self.V[0xF] = self.V[reg2 as usize] >> 7;
                self.V[reg1 as usize] = self.V[reg2 as usize] << 1;
            },

            //
            // Address manipulation
            //
            SetAddr(addr) => self.I = addr,
            AddAddr(reg) => {
                let result = self.I + self.V[reg as usize] as u16;
                self.V[0xF] = if overflow12(result) { 1 } else { 0 };
                self.I = result % 0x1000;
            },
            JumpWithOffset(addr) => self.pc = addr + self.V[0] as u16,

            //
            // Manipulation on multiple bytes
            //
            StoreBcd(reg) => {
                let val = self.V[reg as usize];
                mem.write_byte(self.I, (val / 100) % 10);
                mem.write_byte(self.I+1, (val / 10) % 10);
                mem.write_byte(self.I+2, (val / 1) % 10);
            },

            LoadBytes(reg) => {
                for i in range_inclusive(0, reg as usize) {
                    self.V[i] = mem.read_byte(self.I + i as u16);
                }
            },
            StoreBytes(reg) => {
                for i in range_inclusive(0, reg as usize) {
                    mem.write_byte(self.I + i as u16, self.V[i]);
                }
            },

            //
            // Special 2
            //
            GetRandom(reg, val) => self.V[reg as usize] = self.rng.gen::<u8>() & val,
            Draw(x, y, n) => {
                self.V[0xF] = mem.draw(self.V[x as usize], self.V[y as usize], n, self.I);
            },
            LoadGlyph(reg) => self.I = mem.load_glyph(self.V[reg as usize]),
            ClearScreen => mem.clear_disp(),

            //
            // Keyboard management
            //
            SkipIfKeyPressed(reg) => {
                if mem.is_keydown(self.V[reg as usize]) {
                    self.pc += 2;
                }
            },
            SkipIfKeyNotPressed(reg) => {
                if !mem.is_keydown(self.V[reg as usize]) {
                    self.pc += 2;
                }
            },
            KeyWait(reg) => {
                match mem.get_key() {
                    Some(key) => self.V[reg as usize] = key,
                    None => self.pc -= 2,
                }
            },

            //
            // Timer management
            //
            GetDelay(reg) => self.V[reg as usize] = self.delay,
            SetDelay(reg) => self.delay = self.V[reg as usize],
            SetSound(reg) => self.sound = self.V[reg as usize],
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
