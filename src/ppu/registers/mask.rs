use crate::ppu::PPU;
use crate::lib::get_bit;

enum MaskFlags {
    Greyscale = 0,
    Leftmost8PXLBackground = 1,
    Leftmost8PXLSprite = 2,
    ShowBackground = 3,
    ShowSprites = 4,
    EmphasiseRed = 5,
    EmphasiseGreen = 6,
    EmphasiseBlue = 7,
}

pub enum Color {
    Red,
    Green,
    Blue,
}

impl PPU {
    pub fn is_grayscale(&self) -> bool {
        get_bit(self.mask, MaskFlags::Greyscale as u8)
    }

    pub fn leftmost_8pxl_background(&self) -> bool {
        get_bit(self.mask, MaskFlags::Leftmost8PXLBackground as u8)
    }

    pub fn leftmost_8pxl_sprite(&self) -> bool {
        get_bit(self.mask, MaskFlags::Leftmost8PXLSprite as u8)
    }

    pub fn show_background(&self) -> bool {
        get_bit(self.mask, MaskFlags::ShowBackground as u8)
    }

    pub fn show_sprites(&self) -> bool {
        get_bit(self.mask, MaskFlags::ShowSprites as u8)
    }

    pub fn emphasise(&self) -> Vec<Color> {
        let mut result = Vec::<Color>::new();
        if get_bit(self.mask, MaskFlags::EmphasiseRed as u8) {
            result.push(Color::Red);
        }
        if get_bit(self.mask, MaskFlags::EmphasiseBlue as u8) {
            result.push(Color::Blue);
        }
        if get_bit(self.mask, MaskFlags::EmphasiseGreen as u8) {
            result.push(Color::Green);
        }

        result
    }
} 