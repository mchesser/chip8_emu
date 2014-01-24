pub struct Input {
    repr: [bool,..0x10],
    key: Option<u8>
}

impl Input {
    pub fn new() -> Input {
        Input {
            repr: [false, ..0x10],
            key: None
        }
    }

    pub fn keydown(&self, key_code: u8) -> bool {
        assert!(key_code < 0x10, "Invalid key");
        self.repr[key_code]
    }

    pub fn set_keydown(&mut self, key_code: u8) {
        assert!(key_code < 0x10, "Invalid key");
        self.repr[key_code] = true
    }

    pub fn set_keyup(&mut self, key_code: u8) {
        assert!(key_code < 0x10, "Invalid key");
        self.repr[key_code] = false;
        match self.key {
            Some(k) if k == key_code => self.key = None,
            _ => {}
        }
    }

    pub fn get_key(&mut self) -> Option<u8> {
        let key = self.key;
        self.key = None;
        key
    }
}
