
use crate::{lib::{get_bit, set_bit}, ppu::PPU};

// const NOTUSED_BYTE_FLAG_POSITION: u8 = 0b00000001;
// const NOTUSED2_BYTE_FLAG_POSITION: u8 = 0b00000010;
// const NOTUSED3_BYTE_FLAG_POSITION: u8 = 0b00000100;
// const NOTUSED4_BYTE_FLAG_POSITION: u8 = 0b00001000;
// const NOTUSED5_BYTE_FLAG_POSITION: u8 = 0b00010000;
const SPRITE_OVERFLOW_BYTE_FLAG_POSITION: u8 = 5;
const SPRITE_ZERO_HIT_BYTE_FLAG_POSITION: u8 = 6;
const VBLANK_STARTED_BYTE_FLAG_POSITION: u8 = 7;

impl PPU  {
    pub fn set_vblank_status(&mut self, status: bool) {
        set_bit(&mut self.status, VBLANK_STARTED_BYTE_FLAG_POSITION, status);
    }

    pub fn set_sprite_zero_hit(&mut self, status: bool) {
        set_bit(&mut self.status, SPRITE_ZERO_HIT_BYTE_FLAG_POSITION, status);
    }

    pub fn set_sprite_overflow(&mut self, status: bool) {
        set_bit(&mut self.status, SPRITE_OVERFLOW_BYTE_FLAG_POSITION, status);
    }

    pub fn reset_vblank_status(&mut self) {
        set_bit(&mut self.status, VBLANK_STARTED_BYTE_FLAG_POSITION, false);
    }

    pub fn is_in_vblank(&self) -> bool {
        get_bit(self.status, VBLANK_STARTED_BYTE_FLAG_POSITION)
    }
}