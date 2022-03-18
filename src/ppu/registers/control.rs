
use crate::{lib::{get_bit, set_bit}, ppu::PPU};

const NAMETABLE1_FLAG_BYTE_POSITION: u8 = 0;
const NAMETABLE2_FLAG_BYTE_POSITION: u8 = 1;
const VRAM_ADD_INCREMENT_FLAG_BYTE_POSITION: u8 = 2;
const SPRITE_PATTERN_ADDR_FLAG_BYTE_POSITION: u8 = 3;
const BACKROUND_PATTERN_ADDR_FLAG_BYTE_POSITION: u8 = 4;
const SPRITE_SIZE_FLAG_BYTE_POSITION: u8 = 5;
const MASTER_SLAVE_SELECT_FLAG_BYTE_POSITION: u8 = 6;
const GENERATE_NMI_FLAG_BYTE_POSITION: u8 = 7;


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
        if !get_bit(self.ctrl, VRAM_ADD_INCREMENT_FLAG_BYTE_POSITION) {
            1
        } else {
            32
        }
    }

    pub fn sprt_pattern_addr(&self) -> u16 {
        if !get_bit(self.ctrl, SPRITE_PATTERN_ADDR_FLAG_BYTE_POSITION) {
            0
        } else {
            0x1000
        }
    }

    pub fn bknd_pattern_addr(&self) -> u16 {
        if !get_bit(self.ctrl, BACKROUND_PATTERN_ADDR_FLAG_BYTE_POSITION) {
            0
        } else {
            0x1000
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if !get_bit(self.ctrl, SPRITE_SIZE_FLAG_BYTE_POSITION) {
            8
        } else {
            16
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        if !get_bit(self.ctrl, SPRITE_SIZE_FLAG_BYTE_POSITION) {
            0
        } else {
            1
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        return get_bit(self.ctrl, GENERATE_NMI_FLAG_BYTE_POSITION);
    }
}