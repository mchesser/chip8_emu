use std::mem::transmute;

use sdl2;
use sdl2::event::{Event, poll_event};
use sdl2::keycode::KeyCode;

use sdl2::video::{Window, OPENGL};
use sdl2::video::WindowPos::PosCentered;
use sdl2::render;
use sdl2::render::{Renderer, RenderDriverIndex};
use sdl2::render::{Texture, TextureAccess};
use sdl2::pixels::PixelFormatFlag;

use chip8;
use timer::Timer;

const SCALE: isize = 8;
const WIDTH: isize = chip8::video::WIDTH as isize * SCALE;
const HEIGHT: isize = chip8::video::HEIGHT as isize * SCALE;

const SRC_WIDTH: isize = chip8::video::WIDTH as isize;
const SRC_HEIGHT: isize = chip8::video::HEIGHT as isize;

pub fn run(mut emulator: chip8::Emulator) -> Result<(), String> {
    sdl2::init(sdl2::INIT_EVERYTHING);

    let window = try!(Window::new("CHIP8 Emulator", PosCentered, PosCentered,
        WIDTH, HEIGHT, OPENGL));

    let renderer = try!(Renderer::from_window(window, RenderDriverIndex::Auto,
        render::ACCELERATED));

    let mut emulator_texture = try!(renderer.create_texture(PixelFormatFlag::ARGB8888,
        TextureAccess::Streaming, SRC_WIDTH, SRC_HEIGHT));

    let mut cpu_timer = Timer::new();
    let mut timer = Timer::new();

    'main: loop {
        'event: loop {
            match poll_event() {
                Event::Quit(_) => break 'main,

                Event::KeyDown(_, _, code, _, _, _) => {
                    if let Some(key) = convert_keycode(code) {
                        emulator.keydown(key);
                    }
                }

                Event::KeyUp(_, _, code, _, _, _) => {
                    if let Some(val) = convert_keycode(code) {
                        emulator.keyup(val);
                    }
                }

                Event::None => break,
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
            try!(renderer.clear());
            try!(render_screen(&mut emulator_texture, emulator.display()));
            try!(renderer.copy(&emulator_texture, None, None));

            renderer.present();
        }
    }

    Ok(())
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
        KeyCode::V => Some(0xF),
        _ => None
    }
}

fn render_screen(tex: &mut Texture, chip8_image: &[u8]) -> Result<(), String> {
    // Colors in the format ARGB
    const BLACK: u32 = 0xFF_00_00_00;
    const WHITE: u32 = 0xFF_FF_FF_FF;

    tex.with_lock(None, |mut pixels, _| {
        unsafe {
            let dest: &mut [u32] = transmute(pixels.as_mut_slice());
            let mut offset = 0;
            for &block in chip8_image.iter() {
                for bit in (0..8).rev() {
                    dest[offset] = if is_black(block, bit) { BLACK } else { WHITE };
                    offset += 1;
                }
            }
        }
    })
}

fn is_black(byte: u8, bit: usize) -> bool {
    byte & (0x1 << bit) == 0
}
