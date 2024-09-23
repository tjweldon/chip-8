pub struct Screen {
    pixels: [[u8; 64]; 32]
}

impl Screen {
    pub fn init() -> Self {
        Screen {
            pixels: [[0; 64]; 32]
        }
    }

    fn bit(byte: &u8, idx: usize) -> u8 {
        let Some(idx) = 7usize.checked_sub(idx) else {
            return 0;
        };
        

        (byte >> idx) & 0b0000_0001
    }

    pub fn blit_sprite(&mut self, sprite: &[u8], left_top: (u8, u8)) -> bool {
        let (top, left) = left_top;
        let mut overlap = false;
        for (dy, line) in sprite.iter().enumerate() {
            let y = dy + top as usize;

            for dx in 0..8 {
                let x: usize = (left + dx) as usize;
                let px: &mut u8 = &mut self.pixels[y%32][x%64];
                let bit = Self::bit(line, x);

                if !overlap && x != 0 && *px != 0 {
                    overlap = true;
                }

                *px = (*px ^ bit) & 0b0000_0001;
            }
        }

        overlap
    }

    pub fn clear(&mut self) {
        self.pixels = [[0; 64]; 32];
    }
}
