use crate::cart::Cart;
use crate::ppu::PPU;
use crate::joypad::Joypad;
use crate::state::State;

pub enum GameloopAction {
    NoAction,
    SaveState,
    LoadState
}

pub(crate) trait Memory {
    fn read(&mut self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, value: u8);

    fn read_16(&mut self, pos: u16) -> u16;
  
    fn write_16(&mut self, pos: u16, value: u16);
}

pub struct Bus<'call> {
    vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: PPU,
    joypad: Joypad,
    cycles: usize,

    gameloop_callback: Box<dyn FnMut(&PPU, &mut Joypad) -> GameloopAction + 'call>,
}
 
impl Bus<'_>{
    pub(crate) fn new<'call, F>(cart: Cart, gameloop_callback: F) -> Bus<'call> 
    where
        F: FnMut(&PPU, &mut Joypad) -> GameloopAction + 'call,
    {
        let ppu = PPU::new(cart.chr_rom, cart.rom_header.screen_mirroring);

        Bus {
            vram: [0; 2048],
            prg_rom: cart.prg_rom,
            ppu,
            cycles: 0,
            gameloop_callback: Box::from(gameloop_callback),
            joypad: Joypad::new(),            
        }
    }

    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            //mirror if needed
            addr %= 0x4000;
        }
        self.prg_rom[addr as usize]
    }

    pub(crate) fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;

        let nmi_before = self.ppu.nmi_interrupt.is_some();
        // Cycles multiplied by 3 since the PPU clock runs 3 time faster than CPU clock
        self.ppu.tick(cycles * 3);
        let nmi_after = self.ppu.nmi_interrupt.is_some();

        if !nmi_before && nmi_after {
            let action = (self.gameloop_callback)(&self.ppu, &mut self.joypad);

            match action {
                GameloopAction::NoAction => {},
                GameloopAction::SaveState => self.save_state(),
                GameloopAction::LoadState => self.load_state(),
            }
        }
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu.poll_nmi_interrupt()
    }

    pub fn save_state(&mut self) {
        let state = State::new(self.vram, self.ppu.clone());
        state.save_state(1);
    }

    pub fn load_state(&mut self) {
        let state = State::load_state(1);
        if let Ok(state) = state {
            self.vram = state.ram;
            self.ppu = state.ppu;
        }
    }
}

impl Memory for Bus<'_> {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // RAM Registers
            0x0000 ..= 0x1FFF => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.vram[mirror_down_addr as usize]
            }
            // PPU Registers
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                // panic!("Attempt to read from write-only PPU address {:x}", addr);
                0
            }
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),
            0x2008 ..= 0x3FFF => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.read(mirror_down_addr)
            },
            // APU
            0x4000 ..= 0x4015 => 0, // Ignore APU
            // Joypad Controller
            0x4016 => self.joypad.read(),
            0x4017 => 0, // Second joypad
            // PRG ROM Registers
            0x8000 ..= 0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("Ignoring mem access at 0x{:X}", addr);
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
            0x2000 => {
                /*  
                    https://www.nesdev.org/wiki/PPU_registers
                    To avoid this problem it is prudent to read $2002 immediately before writing $2000 to clear the vblank flag.  
                */
                self.ppu.read_status();
                self.ppu.write_to_ctrl(value);
            },
            0x2001 => self.ppu.write_to_mask(value),
            0x2002 => panic!("attempt to write to PPU status register"),
            0x2003 => self.ppu.write_to_oam_addr(value),
            0x2004 => self.ppu.write_to_oam_data(value),
            0x2005 => self.ppu.write_to_scroll(value),
            0x2006 => self.ppu.write_to_ppu_addr(value),
            0x2007 => self.ppu.write_to_data(value),
            0x2008 ..= 0x3FFF => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.write(mirror_down_addr, value);
            },
            // APU
            0x4000 ..= 0x4013 | 0x4015 => {},
            0x4014 => {
                let mut buffer: [u8; 256] = [0; 256];
                let hi: u16 = (value as u16) << 8;
                for i in 0..256u16 {
                    buffer[i as usize] = self.read(hi + i);
                }

                self.ppu.write_oam_dma(&buffer);
            }
            // Joypad Controllers
            0x4016 => self.joypad.write(value),
            0x4017 =>  { /* Second joypad */ },
            // PRG ROM Registers
            0x8000 ..= 0xFFFF => {
                panic!("Attempt to write to Cartridge ROM space")
            }
            _ => {
                println!("Ignoring mem write-access at 0x{:X}", addr);
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