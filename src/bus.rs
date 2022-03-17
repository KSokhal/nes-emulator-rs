use crate::cart::Cart;

pub(crate) trait Memory {
    fn read(&self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, value: u8);

    fn read_16(&self, pos: u16) -> u16;
  
    fn write_16(&mut self, pos: u16, value: u16);
}


pub struct Bus {
    vram: [u8; 2048],
    rom: Cart,
 }
 
impl Bus {
    pub(crate) fn new(cart: Cart) -> Self {
        Bus {
            vram: [0; 2048],
            rom: cart,            
        }
    }
}

impl Memory for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // RAM Registers
            0x0000 ..= 0x1FFF => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.vram[mirror_down_addr as usize]
            }
            // PPU Registers
            0x2000 ..= 0x3FFF => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                // todo!("PPU is not supported yet")
                0
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
            0x2000 ..= 0x3FFF => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet");
            }
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
    fn read_16(&self, addr: u16) -> u16 {
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


impl Bus {
    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            //mirror if needed
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
  }