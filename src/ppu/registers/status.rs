use crate::lib::{get_bit, set_bit};
use crate::ppu::PPU;

enum StatusFlags {
    SpriteOverflow = 5,
    SpriteZeroHit = 6,
    VBlankStarted = 7,
}

impl PPU  {
    pub fn set_vblank_status(&mut self, status: bool) {
        set_bit(&mut self.status, StatusFlags::VBlankStarted as u8, status);
    }

    pub fn set_sprite_zero_hit(&mut self, status: bool) {
        set_bit(&mut self.status, StatusFlags::SpriteZeroHit as u8, status);
    }

    pub fn set_sprite_overflow(&mut self, status: bool) {
        set_bit(&mut self.status, StatusFlags::SpriteOverflow as u8, status);
    }

    pub fn reset_vblank_status(&mut self) {
        set_bit(&mut self.status, StatusFlags::VBlankStarted as u8, false);
    }

    pub fn is_in_vblank(&self) -> bool {
        get_bit(self.status, StatusFlags::VBlankStarted as u8)
    }
}