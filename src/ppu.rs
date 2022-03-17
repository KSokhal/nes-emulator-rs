use std::ops::Add;

use crate::cart::Mirroring;

const NAMETABLE1_FLAG_BYTE_POSITION: u8 = 0;
const NAMETABLE2_FLAG_BYTE_POSITION: u8 = 1;
const VRAM_ADD_INCREMENT_FLAG_BYTE_POSITION: u8 = 2;
const SPRITE_PATTERN_ADDR_FLAG_BYTE_POSITION: u8 = 3;
const BACKROUND_PATTERN_ADDR_FLAG_BYTE_POSITION: u8 = 4;
const SPRITE_SIZE_FLAG_BYTE_POSITION: u8 = 5;
const MASTER_SLAVE_SELECT_FLAG_BYTE_POSITION: u8 = 6;
const GENERATE_NMI_FLAG_BYTE_POSITION: u8 = 7;

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub addr: AddrRegister,
}


impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom,
            mirroring,
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            addr: AddrRegister::default(),
        }
    }

    fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }
}


pub struct AddrRegister {
    value: (u8, u8),
    hi_ptr: bool,
}
 
impl Default for AddrRegister {
    fn default() -> Self {
        AddrRegister {
            value: (0, 0), // high byte first, lo byte second
            hi_ptr: true,
        }
    }
}

impl AddrRegister {
    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get() > 0x3fff { //mirror down addr above 0x3fff
            self.set(self.get() & 0b11111111111111);
        }
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }
        if self.get() > 0x3fff {
            self.set(self.get() & 0b11111111111111); //mirror down addr above 0x3fff
        }
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }

    pub fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }
}