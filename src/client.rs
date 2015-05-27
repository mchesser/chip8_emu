use std::mem::transmute;

use sdl2;
use sdl2::event::Event;
use sdl2::keycode::KeyCode;

use sdl2::render::{Texture, TextureAccess};
use sdl2::pixels::PixelFormatEnum;

use chip8;
use timer::Timer;

const SCALE: u32 = 8;
const WIDTH: u32 = chip8::video::WIDTH as u32 * SCALE;
const HEIGHT: u32 = chip8::video::HEIGHT as u32 * SCALE;

const SRC_WIDTH: u32 = chip8::video::WIDTH as u32;
const SRC_HEIGHT: u32 = chip8::video::HEIGHT as u32;

pub fn run(mut emulator: chip8::Emulator) -> Result<(), String> {
    let mut sdl_context = sdl2::init().video().unwrap();

    let window = try!(sdl_context.window("CHIP8 Emulator", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build());

    let mut renderer = try!(window.renderer().build());

    let mut emulator_texture = try!(renderer.create_texture(PixelFormatEnum::ARGB8888,
        TextureAccess::Streaming, (SRC_WIDTH as i32, SRC_HEIGHT as i32)));

    let mut cpu_timer = Timer::new();
    let mut timer = Timer::new();
    let mut events = sdl_context.event_pump();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main,

                Event::KeyDown{ keycode: code, ..} => {
                    if let Some(key) = convert_keycode(code) {
                        emulator.keydown(key);
                    }
                }

                Event::KeyUp{ keycode: code, ..} => {
                    if let Some(val) = convert_keycode(code) {
                        emulator.keyup(val);
                    }
                }

                _ => {},
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
            let mut drawer = renderer.drawer();

            drawer.clear();
            render_screen(&mut emulator_texture, emulator.display());
            drawer.copy(&emulator_texture, None, None);
            drawer.present();
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

fn render_screen(tex: &mut Texture, chip8_image: &[u8]) {
    // Colors in the format ARGB
    const BLACK: u32 = 0xFF_00_00_00;
    const WHITE: u32 = 0xFF_FF_FF_FF;

    let _ = tex.with_lock(None, |mut pixels, _| {
        unsafe {
            let dest: &mut [u32] = transmute(&mut *pixels);
            let mut offset = 0;
            for &block in chip8_image {
                for bit in (0..8).rev() {
                    dest[offset] = if is_black(block, bit) { BLACK } else { WHITE };
                    offset += 1;
                }
            }
        }
    });
}

fn is_black(byte: u8, bit: usize) -> bool {
    byte & (0x1 << bit) == 0
}
