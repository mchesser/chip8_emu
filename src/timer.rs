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
        self.last - sdl2::timer::get_ticks()
    }
    
    pub fn reset(&mut self) {
        self.last = sdl2::timer::get_ticks();
    }
}