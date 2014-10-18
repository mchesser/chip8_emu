use chip8::cpu;

pub fn decode(op: u16) -> cpu::Operation {
    match mask01(op) {
        0x0 => {
            match mask13(op) {
                0x0E0 => cpu::ClearScreen,
                0x0EE => cpu::Return,
                addr => cpu::CallRCA(addr),
            }
        },
        0x1 => cpu::Jump(mask13(op)),
        0x2 => cpu::Call(mask13(op)),
        0x3 => cpu::SkipIfEq(mask11(op), cpu::Const(mask22(op))),
        0x4 => cpu::SkipIfNotEq(mask11(op), cpu::Const(mask22(op))),
        0x5 => cpu::SkipIfEq(mask11(op), cpu::Reg(mask21(op))),
        0x6 => cpu::Set(mask11(op), cpu::Const(mask22(op))),
        0x7 => cpu::Add(mask11(op), cpu::Const(mask22(op))),
        0x8 => {
            let r1 = mask11(op);
            let r2 = mask21(op);
            match mask31(op) {
                0x0 => cpu::Set(r1, cpu::Reg(r2)),
                0x1 => cpu::Or(r1, r2),
                0x2 => cpu::And(r1, r2),
                0x3 => cpu::Xor(r1, r2),
                0x4 => cpu::Add(r1, cpu::Reg(r2)),
                0x5 => cpu::Sub(r1, r2),
                0x6 => cpu::Shr(r1, r2),
                0x7 => cpu::SubRev(r1, r2),
                0xE => cpu::Shl(r1, r2),
                _ => fail!("Invalid opcode {:4x}", op)
            }
        },
        0x9 => cpu::SkipIfNotEq(mask11(op), cpu::Reg(mask21(op))),
        0xA => cpu::SetAddr(mask13(op)),
        0xB => cpu::JumpWithOffset(mask13(op)),
        0xC => cpu::GetRandom(mask11(op), mask22(op)),
        0xD => cpu::Draw(mask11(op), mask21(op), mask31(op)),
        0xE => {
            match mask22(op) {
                0x9E => cpu::SkipIfKeyPressed(mask11(op)),
                0xA1 => cpu::SkipIfKeyNotPressed(mask11(op)),
                _ => fail!("Invalid opcode {:4x}", op)
            }
        },
        0xF => {
            match mask22(op) {
                0x07 => cpu::GetDelay(mask11(op)),
                0x0A => cpu::KeyWait(mask11(op)),
                0x15 => cpu::SetDelay(mask11(op)),
                0x18 => cpu::SetSound(mask11(op)),
                0x1E => cpu::AddAddr(mask11(op)),
                0x29 => cpu::LoadGlyph(mask11(op)),
                0x33 => cpu::StoreBcd(mask11(op)),
                0x55 => cpu::StoreBytes(mask11(op)),
                0x65 => cpu::LoadBytes(mask11(op)),
                0x75 => cpu::Unimplemented(op),
                0x85 => cpu::Unimplemented(op),
                _ => fail!("Invalid opcode {:4x}", op)
            }
        },
        _ => unreachable!(),
    }
}

fn mask01(op: u16) -> u8 { ((op & 0xF000) >> 12) as u8 }
fn mask11(op: u16) -> u8 { ((op & 0x0F00) >> 8) as u8 }
fn mask13(op: u16) -> u16 { op & 0x0FFF }
fn mask21(op: u16) -> u8 { ((op & 0x00F0) >> 4) as u8 }
fn mask22(op: u16) -> u8 { (op & 0x00FF) as u8 }
fn mask31(op: u16) -> u8 { (op & 0x000F) as u8 }
