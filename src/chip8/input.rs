use std::mem;

pub struct Input {
    data: [bool; 0x10],
    pressed_key: Option<u8>,
}

impl Input {
    pub fn new() -> Input {
        Input { data: [false; 0x10], pressed_key: None }
    }

    pub fn is_keydown(&self, keycode: u8) -> bool {
        if keycode < 0x10 { self.data[keycode as usize] } else { false }
    }

    pub fn set_keydown(&mut self, keycode: u8) {
        if keycode < 0x10 {
            self.data[keycode as usize] = true;
            self.pressed_key = Some(keycode);
        }
    }

    pub fn set_keyup(&mut self, keycode: u8) {
        if keycode < 0x10 {
            self.data[keycode as usize] = false;
            if self.pressed_key == Some(keycode) {
                self.pressed_key = None;
            }
        }
    }

    pub fn get_key(&mut self) -> Option<u8> {
        mem::replace(&mut self.pressed_key, None)
    }
}
