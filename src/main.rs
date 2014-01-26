extern mod native;
extern mod sdl2;
extern mod extra;

use std::io::File;
use std::os;
use opts = extra::getopts::groups;

mod sdl;
mod chip8;
mod timer;
mod disasm;

#[start]
#[cfg(not(test))]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    let args = os::args();
    let opts = ~[
        opts::optflag("", "disasm", "run disassembler on program")
    ];
    let matches = match opts::getopts(args.tail(), opts) {
        Ok(m)  => { m },
        Err(f) => fail!(f.to_err_msg())
    };

    let mut file = File::open(&Path::new(matches.free[0].as_slice()));

    // Run the dissassembler
    if matches.opt_present("disasm") {
        let disassembly = disasm::disassemble(file.read_to_end());
        for line in disassembly.iter() {
            println!("{}", *line);
        }
    }
    // Run the emulator
    else {
        let mut chip8 = chip8::Chip8::new();
        match file.read(chip8.mem.main) {
            Some(n) => println!("Loaded program of size: {}", n),
            None => fail!("Found empty file")
        }
        sdl::run(chip8);
    }
}
