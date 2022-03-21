use crate::{cart::Cart, ppu::PPU};

pub(crate) trait Memory {
    fn read(&mut self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, value: u8);

    fn read_16(&mut self, pos: u16) -> u16;
  
    fn write_16(&mut self, pos: u16, value: u16);
}


pub struct Bus {
    vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: PPU,
    cycles: usize
 }
 
impl Bus {
    pub(crate) fn new(cart: Cart) -> Self {
        let ppu = PPU::new(cart.chr_rom, cart.rom_header.screen_mirroring);

        Bus {
            vram: [0; 2048],
            prg_rom: cart.prg_rom,
            ppu,
            cycles: 0,            
        }
    }

    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            //mirror if needed
            addr = addr % 0x4000;
        }
        self.prg_rom[addr as usize]
    }

    pub(crate) fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        // Cycles multiplied by 3 since the PPU clock runs 3 time faster than CPU clock
        let _ = self.ppu.tick(cycles * 3);
    }
}

impl Memory for Bus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // RAM Registers
            0x0000 ..= 0x1FFF => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.vram[mirror_down_addr as usize]
            }
            // PPU Registers
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU address {:x}", addr);
                // 0
            }
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),
            0x2008 ..= 0x3FFF => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.read(mirror_down_addr)
            }
            // PRG ROM Registers
            0x8000 ..= 0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("Ignoring mem access at {}", addr);
                0
            }
        }
    }
 
    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM Registers
            0x0000 ..= 0x1FFF => {
                let mirror_down_addr = addr & 0b11111111111;
                self.vram[mirror_down_addr as usize] = value;
            }
            // PPU Registers
            0x2000 => self.ppu.write_to_ctrl(value),
            0x2001 => self.ppu.write_to_mask(value),
            0x2002 => panic!("attempt to write to PPU status register"),
            0x2003 => self.ppu.write_to_oam_addr(value),
            0x2004 => self.ppu.write_to_oam_data(value),
            0x2005 => self.ppu.write_to_scroll(value),
            0x2006 => self.ppu.write_to_ppu_addr(value),
            0x2007 => self.ppu.write_to_data(value),
            0x2008 ..= 0x3FFF => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.write(mirror_down_addr, value);
                // todo!("PPU is not supported yet");
            },
            // PRG ROM Registers
            0x8000 ..= 0xFFFF => {
                panic!("Attempt to write to Cartridge ROM space")
            }
            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        }
    }

    /*
    NES CPU uses Little-Endian addressing rather than Big-Endian.
    That means that the 8 least significant bits of an address will be stored before the 8 most significant bits.
    */
    fn read_16(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn write_16(&mut self, addr: u16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xff) as u8;
        self.write(addr, lo);
        self.write(addr + 1, hi);
    }
}