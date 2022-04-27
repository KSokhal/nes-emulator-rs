use crate::lib::get_bit;
use crate::ppu::PPU;

#[allow(dead_code)]
enum ControlFlags {
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
        if !get_bit(self.ctrl, ControlFlags::VRamAdd as u8) {
            1
        } else {
            32
        }
    }

    pub fn sprt_pattern_addr(&self) -> u16 {
        if !get_bit(self.ctrl, ControlFlags::SpritePatternAddr as u8) {
            0
        } else {
            0x1000
        }
    }

    pub fn bknd_pattern_addr(&self) -> u16 {
        if !get_bit(self.ctrl, ControlFlags::BackgroundPatternAddr as u8) {
            0
        } else {
            0x1000
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if !get_bit(self.ctrl, ControlFlags::SpriteSize as u8) {
            8
        } else {
            16
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        // if !get_bit(self.ctrl, PPUControlFlags::SpriteSize as u8) {
        if !get_bit(self.ctrl, ControlFlags::MasterSlaveSelect as u8) {
            0
        } else {
            1
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        get_bit(self.ctrl, ControlFlags::GenerateNMI as u8)
    }
}