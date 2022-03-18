use crate::ppu::PPU;
use crate::lib::get_bit;


const GREYSCALE_BYTE_FLAG_POSITION: u8 = 0;
const LEFTMOST_8PXL_BACKGROUND_BYTE_FLAG_POSITION: u8 = 1;
const LEFTMOST_8PXL_SPRITE_BYTE_FLAG_POSITION: u8 = 2;
const SHOW_BACKGROUND_BYTE_FLAG_POSITION: u8 = 3;
const SHOW_SPRITES_BYTE_FLAG_POSITION: u8 = 4;
const EMPHASISE_RED_BYTE_FLAG_POSITION: u8 = 5;
const EMPHASISE_GREEN_BYTE_FLAG_POSITION: u8 = 6;
const EMPHASISE_BLUE_BYTE_FLAG_POSITION: u8 = 7;

pub enum Color {
    Red,
    Green,
    Blue,
}

impl PPU {
    pub fn is_grayscale(&self) -> bool {
        get_bit(self.mask, GREYSCALE_BYTE_FLAG_POSITION)
    }

    pub fn leftmost_8pxl_background(&self) -> bool {
        get_bit(self.mask, LEFTMOST_8PXL_BACKGROUND_BYTE_FLAG_POSITION)
    }

    pub fn leftmost_8pxl_sprite(&self) -> bool {
        get_bit(self.mask, LEFTMOST_8PXL_SPRITE_BYTE_FLAG_POSITION)
    }

    pub fn show_background(&self) -> bool {
        get_bit(self.mask, SHOW_BACKGROUND_BYTE_FLAG_POSITION)
    }

    pub fn show_sprites(&self) -> bool {
        get_bit(self.mask, SHOW_SPRITES_BYTE_FLAG_POSITION)
    }

    pub fn emphasise(&self) -> Vec<Color> {
        let mut result = Vec::<Color>::new();
        if get_bit(self.mask, EMPHASISE_RED_BYTE_FLAG_POSITION) {
            result.push(Color::Red);
        }
        if get_bit(self.mask, EMPHASISE_BLUE_BYTE_FLAG_POSITION) {
            result.push(Color::Blue);
        }
        if get_bit(self.mask, EMPHASISE_GREEN_BYTE_FLAG_POSITION) {
            result.push(Color::Green);
        }

        result
    }
} 