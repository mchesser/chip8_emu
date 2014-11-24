use std::mem::transmute;

use sdl2;
use sdl2::event;
use sdl2::event::poll_event;
use sdl2::keycode::KeyCode;
use sdl2::video::{Window, PosCentered, OPENGL};
use sdl2::surface::Surface;

use chip8;
use timer::Timer;

const SCALE: int = 8;
const WIDTH: int = chip8::video::WIDTH as int * SCALE;
const HEIGHT: int = chip8::video::HEIGHT as int * SCALE;

const SRC_WIDTH: uint = chip8::video::BYTES_WIDTH as uint;
const SRC_HEIGHT: uint = chip8::video::HEIGHT as uint;

pub fn run(mut emulator: chip8::Emulator) {
    sdl2::init(sdl2::INIT_EVERYTHING);

    let window = match Window::new("CHIP8 Emulator", PosCentered, PosCentered,
        WIDTH, HEIGHT, OPENGL)
    {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let mut surface = match window.get_surface() {
        Ok(surface) => surface,
        Err(err) => panic!(format!("failed to get window surface: {}", err))
    };

    let mut cpu_timer = Timer::new();
    let mut timer = Timer::new();

    'main: loop {
        'event: loop {
            match event::poll_event() {
                event::Quit(_) => break 'main,

                event::KeyDown(_, _, code, _, _, _) => {
                    if let Some(key) = convert_keycode(code) {
                        emulator.keydown(key);
                    }
                }

                event::KeyUp(_, _, code, _, _, _) => {
                    if let Some(val) = convert_keycode(code) {
                        emulator.keyup(val);
                    }
                }

                event::None => break,
                _ => continue,
            }
        }

        if cpu_timer.elapsed_seconds() >= chip8::TICK_RATE {
            cpu_timer.reset();
            emulator.tick();
        }

        if timer.elapsed_seconds() >= chip8::CLOCK_RATE {
            timer.reset();
            emulator.frame();
        }

        if emulator.poll_screen() {
            render_screen(&mut surface, emulator.display());
            window.update_surface();
        }
    }
}

fn convert_keycode(code: KeyCode) -> Option<u8> {
    // ------------
    // 1234    123C
    // QWER => 456D
    // ASDF    789E
    // ZXCV    A0BF
    // ------------
    match code {
        KeyCode::Num1 => Some(0x1),
        KeyCode::Num2 => Some(0x2),
        KeyCode::Num3 => Some(0x3),
        KeyCode::Num4 => Some(0xC),
        KeyCode::Q => Some(0x4),
        KeyCode::W => Some(0x5),
        KeyCode::E => Some(0x6),
        KeyCode::R => Some(0xD),
        KeyCode::A => Some(0x7),
        KeyCode::S => Some(0x8),
        KeyCode::D => Some(0x9),
        KeyCode::F => Some(0xE),
        KeyCode::Z => Some(0xA),
        KeyCode::X => Some(0x0),
        KeyCode::C => Some(0xB),
        KeyCode::V=> Some(0xF),
        _ => None
    }
}

fn render_screen(surface: &mut Surface, chip8_image: &[u8]) {
    // Colors in the format ARGB
    static BLACK: u32 = 0x00_00_00_00;
    static WHITE: u32 = 0x00_FF_FF_FF;

    surface.with_lock(|pixels| {
        unsafe {
            let mut dest: *mut u32 = transmute(&pixels[0]);
            let src: *const u8 = transmute(&chip8_image[0]);

            for src_y in range(0, SRC_HEIGHT) {
                for _ in range(0, SCALE) {
                    for src_x in range(0, SRC_WIDTH) {
                        let row = src.offset((src_x + src_y * SRC_WIDTH) as int);
                        for xx in range(0, 8).rev() {
                            let pixel = if is_black(*row, xx) { BLACK } else { WHITE };
                            for _ in range(0, SCALE) {
                                *dest = pixel;
                                dest = dest.offset(1);
                            }
                        }
                    }
                }
            }
        }
    });
}

fn is_black(byte: u8, bit: uint) -> bool {
    byte & (0x1 << bit) == 0
}
