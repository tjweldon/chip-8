use crate::mem::Addr;

pub struct Keyboard;

impl Keyboard {
    pub fn key_is_pressed(&self, key: Addr<u8, 16>) -> bool { false }
}
