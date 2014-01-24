use chip8::Chip8;
use chip8video = chip8::video;

use sdl2;
use sdl2::{event, video, keycode};
use sdl2::surface::Surface;
use std::cast::transmute;

pub fn run(chip8: Chip8) {
    let mut chip8 = chip8;
    
    sdl2::init([sdl2::InitVideo]);
    
    // Initialise the window
    let window =
        match video::Window::new("CHIP8 Emulator", video::PosCentered,
            video::PosCentered, chip8video::WIDTH as int, chip8video::HEIGHT as int, [video::OpenGL, video::Borderless]) {
            Ok(window) => window,
            Err(err) => fail!(format!("failed to create window: {}", err))
    };
    
    // Get the surface
    let mut surface =
        match window.get_surface() {
            Ok(surface) => surface,
            Err(err) => fail!(format!("failed to get window surface: {}", err))
    };
    
    
    'main: loop {
        'event: loop {
            match event::poll_event() {
                event::QuitEvent(_) => break 'main,
                
                event::KeyDownEvent(_, _, code, _, _) => {
                    match to_u8(code) {
                        Some(val) => chip8.keydown(val),
                        None      => {}
                    }
                }
                
                event::KeyUpEvent(_, _, code, _, _) => {
                    match to_u8(code) {
                        Some(val) => chip8.keyup(val),
                        None      => {}
                    }
                }
                
                event::NoEvent => break,
                _ => {}            
            }
        }
        
        chip8.frame();
        
        render_chip8_screen(surface, chip8.image());
        window.update_surface();
    }
}

fn to_u8(code: keycode::KeyCode) -> Option<u8> {
    match code {
        keycode::Num0Key => Some(0x0),
        keycode::Num1Key => Some(0x1),
        keycode::Num2Key => Some(0x2),
        keycode::Num3Key => Some(0x3),
        keycode::Num4Key => Some(0x4),
        keycode::Num5Key => Some(0x5),
        keycode::Num6Key => Some(0x6),
        keycode::Num7Key => Some(0x7),
        keycode::Num8Key => Some(0x8),
        keycode::Num9Key => Some(0x9),
        keycode::AKey    => Some(0xA),
        keycode::BKey    => Some(0xB),
        keycode::CKey    => Some(0xC),
        keycode::DKey    => Some(0xD),
        keycode::EKey    => Some(0xE),
        keycode::FKey    => Some(0xF),
        _ => None
    }
}

fn render_chip8_screen(surface: &mut Surface, chip8_image: &[u8]) {
    // Colors in the format ARGB
    static BLACK: u32 = 0x00_00_00_00;
    static WHITE: u32 = 0x00_FF_FF_FF;
    
    surface.with_lock(|pixels| {
        unsafe {
            let mut dest: *mut u32 = transmute(&pixels[0]);
            let src: *u8 = transmute(&chip8_image[0]);
            
            for src_y in range(0, chip8video::HEIGHT) {
                for src_x in range(0, chip8video::REPWIDTH) {
                    let row = src.offset((src_x + src_y * chip8video::REPWIDTH) as int);
                    for xx in range(0, 8) {
                        let pixel = if *row & (0x1 << xx) == 0 { BLACK } else { WHITE };
                        // TODO: scale pixels
                        *dest = pixel;
                        dest = dest.offset(1);
                    }
                }
            }
        }
    });
} 