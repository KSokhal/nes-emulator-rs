use crate::cart::Mirroring;

use registers::addr::AddrRegister;

use self::registers::scroll::ScrollRegister;
pub mod registers;

pub(crate) struct PPU {
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub ctrl: u8,
    pub mask: u8,
    pub status: u8,
    pub scroll: ScrollRegister,
    pub addr: AddrRegister,
    pub vram: [u8; 2048],

    pub oam_addr: u8,
    pub oam_data: [u8; 256],
    pub palette_table: [u8; 32],
  
    internal_data_buffer: u8,
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
            ctrl: 0,
            internal_data_buffer: 0,
            mask: 0,
            status: 0,
            scroll: ScrollRegister::default(),
            oam_addr: 0,
        }
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.vram_addr_increment());
    }

    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400; // to the name table index
        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }
    
    pub(crate) fn write_to_ctrl(&mut self, value: u8) {
        let before_nmi_status = self.generate_vblank_nmi();
        self.ctrl = value;
    }

    pub(crate) fn write_to_mask(&mut self, value: u8) {
        self.mask = value;
    }

    pub(crate) fn read_status(&mut self) -> u8 {
        let data = self.status.clone();
        self.reset_vblank_status();
        self.addr.reset_latch();
        self.scroll.reset_latch();
        data
    }

    pub(crate) fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    pub(crate) fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub(crate) fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    pub(crate) fn write_to_scroll(&mut self, value: u8) {
        self.scroll.write(value);
    }

    pub(crate) fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub(crate) fn write_to_data(&mut self, value: u8) {
        let addr = self.addr.get();
        match addr {
            0 ..= 0x1FFF => println!("attempt to write to chr rom space {}", addr), 
            0x2000..=0x2FFF => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000 ..= 0x3EFF => unimplemented!("addr {} shouldn't be used in reallity", addr),

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = value;
            }
            0x3F00 ..= 0x3FFF =>
            {
                self.palette_table[(addr - 0x3f00) as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
        self.increment_vram_addr();
    }

    pub(crate) fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();

        self.increment_vram_addr();

        match addr {
            0 ..= 0x1FFF => {
                let result = self.internal_data_buffer;
                self.internal_data_buffer = self.chr_rom[addr as usize];
                result
            }
            0x2000 ..= 0x2FFF => {
                let result = self.internal_data_buffer;
                self.internal_data_buffer = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000 ..=   0x3EFF => unimplemented!("addr {} shouldn't be used in reallity", addr),

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize]
            }

            0x3f00..=0x3fff =>
            {
                self.palette_table[(addr - 0x3f00) as usize]
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for x in data.iter() {
            self.oam_data[self.oam_addr as usize] = *x;
            self.oam_addr = self.oam_addr.wrapping_add(1);
        }
    }
}