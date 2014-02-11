extern mod native;
extern mod sdl2;
extern mod getopts;

use std::io::File;
use getopts::{optflag,getopts};
use std::os;

mod sdl;
mod chip8;
mod timer;

#[start]
#[cfg(not(test))]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    let args = os::args();
    let opts = ~[
        optflag("", "disasm", "run disassembler on program")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m)  => { m },
        Err(f) => fail!(f.to_err_msg())
    };

    if matches.free.len() != 1 {
        println!("Invalid usage");
        return;
    }

    let mut file = match File::open(&Path::new(matches.free[0].as_slice())) {
        Ok(f) => f,
        Err(err) => fail!("Failed to open input program: {:?}", err)
    };

    // Run the dissassembler
    if matches.opt_present("disasm") {
        let input = match file.read_to_end() {
            Ok(s) => s,
            Err(err) => fail!("Failed to read file: {:?}", err)
        };
        let disassembly = chip8::disasm::disassemble(input);
        for line in disassembly.iter() {
            println!("{}", *line);
        }
    }
    // Run the emulator
    else {
        let mut chip8 = chip8::Chip8::new();
        match file.read(chip8.mem.main) {
            Ok(n) => println!("Loaded program of size: {}", n),
            Err(err) => fail!("Failed to read file: {:?}", err)
        }
        sdl::run(chip8);
    }
}
