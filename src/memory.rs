
pub(crate) struct Memory {
    pub memory: [u8; 0xFFFF],
}

impl Default for Memory {
    fn default() -> Self {
        Self{
            memory: [0; 0xFFFF],
        }
    }
}
impl Memory {
    pub(crate) fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    /*
    NES CPU uses Little-Endian addressing rather than Big-Endian.
    That means that the 8 least significant bits of an address will be stored before the 8 most significant bits.
    */
    pub(crate) fn read_16(&self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub(crate) fn write_16(&mut self, addr: u16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xff) as u8;
        self.write(addr, lo);
        self.write(addr + 1, hi);
    }
}