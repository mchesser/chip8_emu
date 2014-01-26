/// Disassembles a program
pub fn disassemble(prog: &[u8]) -> ~[~str] {
    prog.chunks(2)
        .map(|data| translate((data[0] as u16 << 8) | data[1] as u16))
        .collect::<~[~str]>()
}

/// Translates OP Code into meaningful expressions
pub fn translate(op: u16) -> ~str {
    match mask01(op) {
            0x0 => {
                match op {
                    0x00E0 => format!("CLR"),
                    0x00EE => format!("RET"),
                    _      => format!("FAIL (OPCODE MISSING {:4x})", op)
                }
            },
            0x1 => format!("JMP 0x{:03x}", mask13(op)),
            0x2 => format!("CLL 0x{:03x}", mask13(op)),
            0x3 => format!("IF V{:1x}, {}", mask11(op), mask22(op)),
            0x4 => format!("IF NOT V{:1x}, {}", mask11(op), mask22(op)),
            0x5 => format!("IF V{:1x}, V{}", mask11(op), mask22(op)),
            0x6 => format!("SET V{:1x}, {}", mask11(op), mask22(op)),
            0x7 => format!("ADD V{:1x}, {}", mask11(op), mask22(op)),
            0x8 => {
                let r1 = mask11(op);
                let r2 = mask21(op);
                match mask31(op) {
                    0x0 => format!("SET V{:1x}, V{:1x}", r1, r2),
                    0x1 => format!("OR  V{:1x}, V{:1x}", r1, r2),
                    0x2 => format!("AND V{:1x}, V{:1x}", r1, r2),
                    0x3 => format!("XOR V{:1x}, V{:1x}", r1, r2),
                    0x4 => format!("ADD V{:1x}, V{:1x}", r1, r2),
                    0x5 => format!("SUB V{:1x}, V{:1x}", r1, r2),
                    0x6 => format!("SHR V{:1x}, V{:1x}", r1, r2),
                    0x7 => format!("SU2 V{:1x}, V{:1x}", r1, r2),
                    0xE => format!("SHL V{:1x}, V{:1x}", r1, r2),
                    _   => format!("FAIL (OPCODE INVALID {:4x})", op)
                }
            },
            0x9 => format!("IF NOT V{:1x}, V{:1x}", mask11(op), mask21(op)),
            0xA => format!("ADR 0x{:03x}", mask13(op)),
            0xB => format!("JMP 0x{:03x}, V0", mask13(op)),
            0xC => format!("RND V{:1x}, 0x{:02x}", mask11(op), mask22(op)),
            0xD => format!("DRW V{:1x}, V{:1x}, {}", mask11(op), mask21(op), mask31(op)),
            0xE => {
                match mask22(op) {
                    0x9E => format!("KEY {}", mask11(op)),
                    0xA1 => format!("KEY NOT {}", mask11(op)),
                    _    => format!("FAIL (OPCODE INVALID {:4x})", op)
                }
            },
            0xF => {
                match mask22(op) {
                    0x07 => format!("GETDELAY V{:1x}", mask11(op)),
                    0x0A => format!("KEYWAIT V{:1x}", mask11(op)),
                    0x15 => format!("SETDELAY V{:1x}", mask11(op)),
                    0x18 => format!("SETSOUND V{:1x}", mask11(op)),
                    0x1E => format!("ADDR V{:1x}", mask11(op)),
                    0x29 => format!("FONT V{:1x}", mask11(op)),
                    0x33 => format!("BCD V{:1x}", mask11(op)),
                    0x55 => format!("WRITE V{:1x}", mask11(op)),
                    0x65 => format!("READ V{:1x}", mask11(op)),
                    0x75 => format!("FAIL (OPCODE MISSING {:4x})", op),
                    0x85 => format!("FAIL (OPCODE MISSING {:4x})", op),
                    _    => format!("FAIL (OPCODE INVALID {:4x})", op)
                }
            },
            
            _ => fail!("Unreachable code")
        }
}

fn mask01(op: u16) -> u8 { ((op & 0xF000) >> 12) as u8 }
fn mask11(op: u16) -> u8 { ((op & 0x0F00) >> 8) as u8 }
fn mask13(op: u16) -> u16 { op & 0x0FFF }
fn mask21(op: u16) -> u8 { ((op & 0x00F0) >> 4) as u8 }
fn mask22(op: u16) -> u8 { (op & 0x00FF) as u8 }
fn mask31(op: u16) -> u8 { (op & 0x000F) as u8 }