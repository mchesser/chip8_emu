use macroquad::{miniquad::EventHandler, prelude::*, texture};

use crate::chip8;

const SCALE: u32 = 8;
const WIDTH: u32 = chip8::video::WIDTH as u32 * SCALE;
const HEIGHT: u32 = chip8::video::HEIGHT as u32 * SCALE;

const SRC_WIDTH: u32 = chip8::video::WIDTH as u32;
const SRC_HEIGHT: u32 = chip8::video::HEIGHT as u32;

struct Chip8EventHandler<'a> {
    emulator: &'a mut chip8::Emulator,
}

impl<'a> EventHandler for Chip8EventHandler<'a> {
    fn update(&mut self, _ctx: &mut macroquad::miniquad::Context) {}
    fn draw(&mut self, _ctx: &mut macroquad::miniquad::Context) {}

    fn key_up_event(
        &mut self,
        _ctx: &mut macroquad::miniquad::Context,
        keycode: KeyCode,
        _keymods: macroquad::miniquad::KeyMods,
    ) {
        eprintln!("keyup: {keycode:?}");
        if let Some(key) = convert_keycode(keycode) {
            self.emulator.keyup(key)
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut macroquad::miniquad::Context,
        keycode: KeyCode,
        _keymods: macroquad::miniquad::KeyMods,
        _repeat: bool,
    ) {
        eprintln!("keydown: {keycode:?}");
        if let Some(key) = convert_keycode(keycode) {
            self.emulator.keydown(key)
        }
    }
}

pub async fn run(mut emulator: chip8::Emulator) -> Result<(), String> {
    macroquad::window::request_new_screen_size(WIDTH as f32, HEIGHT as f32);

    let mut screen = Image::gen_image_color(SRC_WIDTH as u16, SRC_HEIGHT as u16, WHITE);
    let screen_texture = texture::render_target(SRC_WIDTH, SRC_HEIGHT).texture;
    screen_texture.set_filter(FilterMode::Nearest);

    let mut timers = Timers::default();

    let events_subscriber = utils::register_input_subscriber();

    loop {
        utils::repeat_all_miniquad_input(
            &mut Chip8EventHandler { emulator: &mut emulator },
            events_subscriber,
        );

        timers.elapsed(get_frame_time() as f64);
        loop {
            match timers.next() {
                TimeEvent::Tick => emulator.tick(),
                TimeEvent::Cycle => emulator.frame(),
                TimeEvent::None => break,
            }
        }

        if emulator.poll_screen() {
            render_screen(&mut screen, emulator.display());
            screen_texture.update(&screen);
        }

        draw_texture_ex(screen_texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some([WIDTH as f32, HEIGHT as f32].into()),
            ..Default::default()
        });

        next_frame().await
    }
}

enum TimeEvent {
    Tick,
    Cycle,
    None,
}

#[derive(Default)]
struct Timers {
    tick: f64,
    cycle: f64,
}

impl Timers {
    pub fn next(&mut self) -> TimeEvent {
        if self.tick < self.cycle && self.tick < 0.0 {
            self.tick += chip8::TICK_RATE;
            TimeEvent::Tick
        }
        else if self.cycle < 0.0 {
            self.cycle += chip8::CLOCK_RATE;
            TimeEvent::Cycle
        }
        else {
            TimeEvent::None
        }
    }

    pub fn elapsed(&mut self, time: f64) {
        self.tick -= time;
        self.cycle -= time;
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
        KeyCode::Key1 => Some(0x1),
        KeyCode::Key2 => Some(0x2),
        KeyCode::Key3 => Some(0x3),
        KeyCode::Key4 => Some(0xC),
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
        _ => None,
    }
}

fn render_screen(dst: &mut Image, chip8_image: &[u8]) {
    let dest: &mut [[u8; 4]] = dst.get_image_data_mut();
    let mut offset = 0;
    for &block in chip8_image {
        for bit in (0..8).rev() {
            dest[offset] = if is_black(block, bit) { BLACK.into() } else { WHITE.into() };
            offset += 1;
        }
    }
}

fn is_black(byte: u8, bit: usize) -> bool {
    byte & (0x1 << bit) == 0
}
