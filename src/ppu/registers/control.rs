use crate::lib::get_bit;
use crate::ppu::PPU;

// const NAMETABLE1_FLAG_BYTE_POSITION: u8 = 0;
// const NAMETABLE2_FLAG_BYTE_POSITION: u8 = 1;
// const VRAM_ADD_INCREMENT_FLAG_BYTE_POSITION: u8 = 2;
// const SPRITE_PATTERN_ADDR_FLAG_BYTE_POSITION: u8 = 3;
// const BACKROUND_PATTERN_ADDR_FLAG_BYTE_POSITION: u8 = 4;
// const SPRITE_SIZE_FLAG_BYTE_POSITION: u8 = 5;
// const MASTER_SLAVE_SELECT_FLAG_BYTE_POSITION: u8 = 6;
// const GENERATE_NMI_FLAG_BYTE_POSITION: u8 = 7;

enum PPUControlFlags {
    Nametable1 = 0,
    Nametable2 = 1,
    VRamAdd = 2,
    SpritePatternAddr = 3,
    BackgroundPatternAddr = 4,
    SpriteSize = 5,
    MasterSlaveSelect = 6,
    GenerateNMI = 7,
}

impl PPU {
    pub fn nametable_addr(&self) -> u16 {
        match self.ctrl & 0b11 {
            0 => 0x2000,
            1 => 0x2400, 
            2 => 0x2800,
            3 => 0x2c00,
            _ => panic!("not possible"),
        }
    }

    pub fn vram_addr_increment(&self) -> u8 {   
        if !get_bit(self.ctrl, PPUControlFlags::VRamAdd as u8) {
            1
        } else {
            32
        }
    }

    pub fn sprt_pattern_addr(&self) -> u16 {
        if !get_bit(self.ctrl, PPUControlFlags::SpritePatternAddr as u8) {
            0
        } else {
            0x1000
        }
    }

    pub fn bknd_pattern_addr(&self) -> u16 {
        if !get_bit(self.ctrl, PPUControlFlags::BackgroundPatternAddr as u8) {
            0
        } else {
            0x1000
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if !get_bit(self.ctrl, PPUControlFlags::SpriteSize as u8) {
            8
        } else {
            16
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        // if !get_bit(self.ctrl, PPUControlFlags::SpriteSize as u8) {
        if !get_bit(self.ctrl, PPUControlFlags::MasterSlaveSelect as u8) {
            0
        } else {
            1
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        return get_bit(self.ctrl, PPUControlFlags::GenerateNMI as u8);
    }
}