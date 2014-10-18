use sdl2;

pub struct Timer {
    last: uint
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            last: sdl2::timer::get_ticks()
        }
    }

    pub fn elapsed(&self) -> uint {
        sdl2::timer::get_ticks() - self.last
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed() as f32 / 1000.0
    }

    pub fn reset(&mut self) {
        self.last = sdl2::timer::get_ticks();
    }
}
