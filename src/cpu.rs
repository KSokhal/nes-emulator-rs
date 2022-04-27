use crate::bus::{Bus, Memory};
use crate::instructions::STACK_RESET;
use crate::registers::{Registers, CPUStatusFlags};
use crate::lib::set_bit;

#[derive(Debug)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPageX,
   ZeroPageY,
   Absolute,
   AbsoluteX,
   AbsoluteY,
   IndirectX,
   IndirectY,
   NoneAddressing,
}

#[allow(clippy::upper_case_acronyms)]
pub(crate) struct CPU<'a> {
    pub regs: Registers,
    pub bus: Bus<'a>,
}

impl CPU<'_> {
    pub fn new(bus: Bus<'_>) -> CPU<'_> {
        CPU { 
            regs: Registers::default(),
            bus
        }
    }

    pub fn reset(&mut self) {
        self.regs.a = 0;
        self.regs.x = 0;
        self.regs.y = 0;
        self.regs.sp = STACK_RESET;
        self.regs.p = 0b100100;
        self.regs.pc = self.read_16(0xFFFC);
    }

    pub fn run<F>(&mut self, mut callback: F) 
    where 
        F: FnMut(&mut CPU),
    {
        loop {
            if let Some(_nmi) = self.bus.poll_nmi_status() {
                self.interrupt_nmi();
            };

            callback(self);

            let opscode = self.read(self.regs.pc);
            self.regs.pc += 1;
            let program_counter_state = self.regs.pc;

            let instruction = self.get_instruction(opscode);
            
            match opscode {
                0x00 => return,
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(&instruction.addr_mode),
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&instruction.addr_mode),
                0x0A => self.asl_accumulator(),
                0x06 | 0x16 | 0x0E | 0x1E => { self.asl(&instruction.addr_mode); },
                0x90 => self.bcc(),
                0xB0 => self.bcs(),
                0xF0 => self.beq(),
                0x24 | 0x2C => self.bit(&instruction.addr_mode),
                0x30 => self.bmi(),
                0xD0 => self.bne(),
                0x10 => self.bpl(),
                0x50 => self.bvc(),
                0x70 => self.bvs(),
                0x18 => self.clc(),
                0xD8 => self.cld(),
                0x58 => self.cli(),
                0xB8 => self.clv(),
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.compare(&instruction.addr_mode, self.regs.a),
                0xE0 | 0xE4 | 0xEC => self.compare(&instruction.addr_mode, self.regs.x),
                0xC0 | 0xC4 | 0xCC => self.compare(&instruction.addr_mode, self.regs.y),
                0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&instruction.addr_mode),
                0xCA => self.dex(),
                0x88 => self.dey(),
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(&instruction.addr_mode),
                0xE6 | 0xF6 | 0xEE | 0xFE => { self.inc(&instruction.addr_mode); },
                0xE8 => self.inx(),
                0xC8 => self.iny(),
                0x4C => self.jmp(),
                0x6C => self.jmp_indirect(),
                0x20 => self.jsr(),
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&instruction.addr_mode),
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&instruction.addr_mode),
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&instruction.addr_mode),
                0x4A => self.lsr_accumulator(),
                0x46 | 0x56 | 0x4E | 0x5E => { self.lsr(&instruction.addr_mode); },
                0xEA => {},
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(&instruction.addr_mode),
                0x48 => self.pha(),
                0x08 => self.php(),
                0x68 => self.pla(),
                0x28 => self.plp(),
                0x2A => self.rol_accumulator(),
                0x26 | 0x36 | 0x2E | 0x3E => { self.rol(&instruction.addr_mode); },
                0x6A => self.ror_accumulator(),
                0x66 | 0x76 | 0x6E | 0x7E => { self.ror(&instruction.addr_mode); },
                0x40 => self.rti(),
                0x60 => self.rts(),
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(&instruction.addr_mode),
                0x38 => set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, true),
                0xF8 => set_bit(&mut self.regs.p, CPUStatusFlags::DecimalMode as u8, true),
                0x78 => set_bit(&mut self.regs.p, CPUStatusFlags::InterruptDisable as u8, true),
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&instruction.addr_mode),
                0x86 | 0x96 | 0x8E => self.stx(&instruction.addr_mode),
                0x84 | 0x94 | 0x8C => self.sty(&instruction.addr_mode),
                0xAA => self.tax(),
                0xA8 => self.tay(),
                0xBA => self.tsx(),
                0x8A => self.txa(),
                0x9A => self.txs(),
                0x98 => self.tya(),
                
                // Unofficial Opcodes
                0xC7 | 0xD7 | 0xCF | 0xDF | 0xDB | 0xD3 | 0xC3 => self.dcp(&instruction.addr_mode),
                0x27 | 0x37 | 0x2F | 0x3F | 0x3B | 0x33 | 0x23 => self.rla(&instruction.addr_mode),
                0x07 | 0x17 | 0x0F | 0x1F | 0x1B | 0x03 | 0x13 => self.slo(&instruction.addr_mode),
                0x47 | 0x57 | 0x4F | 0x5f | 0x5b | 0x43 | 0x53 => self.sre(&instruction.addr_mode),
                
                /* SKB */
                0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => {
                    /* 2 byte NOP (immidiate ) */
                    // todo: might be worth doing the read
                }
                0xCB => self.axs(&instruction.addr_mode),
                0x6B => self.arr(&instruction.addr_mode),
                0xEB => self.sbc_unofficial(&instruction.addr_mode),
                0x0B | 0x2B => self.anc(&instruction.addr_mode),
                0x4B => self.alr(&instruction.addr_mode),

                /* NOP read */
                0x04 | 0x44 | 0x64 | 0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 | 0x0C | 0x1C
                | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => {
                    let (addr, page_crossed) = self.get_op_addr(&instruction.addr_mode);
                    let _value = self.read(addr);

                    if page_crossed {
                        self.bus.tick(1);
                    }
                    /* do nothing */
                },

                0x67 | 0x77 | 0x6F | 0x7F | 0x7B | 0x63 | 0x73 => self.rra(&instruction.addr_mode),
                0xE7 | 0xF7 | 0xEF | 0xFF | 0xFB | 0xE3 | 0xF3 => self.isb(&instruction.addr_mode),
                0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => { },
                0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => { },
                0xA7 | 0xB7 | 0xAF | 0xBF | 0xA3 | 0xB3 => self.lax(&instruction.addr_mode),
                0x87 | 0x97 | 0x8F | 0x83 => self.sax(&instruction.addr_mode),
                0xAB => self.lxa(&instruction.addr_mode),
                0x8B => self.xaa(&instruction.addr_mode),
                0xBB => self.las(&instruction.addr_mode),
                0x9B => self.tas(),
                0x93 => self.ahx_indir_y(),
                0x9F => self.ahx_abs_y(),
                0x9E => self.shx(),
                0x9C => self.shy(),
            }

            self.bus.tick(instruction.cycles);

            if program_counter_state == self.regs.pc {
                self.regs.pc += (instruction.bytes - 1) as u16;
            };
        };
    }

    pub(crate) fn get_op_addr(&mut self, mode: &AddressingMode) -> (u16, bool) {
        match mode {
            AddressingMode::Immediate => (self.regs.pc, false),
            AddressingMode::ZeroPage  => (self.read(self.regs.pc) as u16, false),
            AddressingMode::Absolute => (self.read_16(self.regs.pc), false),
            AddressingMode::ZeroPageX => {
                let pos = self.read(self.regs.pc);
                let addr = pos.wrapping_add(self.regs.x) as u16;
                (addr, false)
            },
            AddressingMode::ZeroPageY => {
                let pos = self.read(self.regs.pc);
                let addr = pos.wrapping_add(self.regs.y) as u16;
                (addr, false)
            },
            AddressingMode::AbsoluteX => {
                let base = self.read_16(self.regs.pc);
                let addr = base.wrapping_add(self.regs.x as u16);

                let page_crossed = base & 0xFF00 != addr & 0xFF00;
                (addr, page_crossed)
            },
            AddressingMode::AbsoluteY => {
                let base = self.read_16(self.regs.pc);
                let addr = base.wrapping_add(self.regs.y as u16);

                let page_crossed = base & 0xFF00 != addr & 0xFF00;
                (addr, page_crossed)
            },
            AddressingMode::IndirectX => {
                let base = self.read(self.regs.pc);
 
                let ptr: u8 = (base as u8).wrapping_add(self.regs.x);
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);
                ((hi as u16) << 8 | (lo as u16), false)
            },
            AddressingMode::IndirectY => {
                let base = self.read(self.regs.pc);
 
                let lo = self.read(base as u16);
                let hi = self.read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.regs.y as u16);

                let page_crossed = deref & 0xFF00 != deref_base & 0xFF00;
                (deref, page_crossed)
            },
            AddressingMode::NoneAddressing => {
                panic!("Addressing mode {:?} is not supported", mode);
            }
        }
    }

    fn interrupt_nmi(&mut self) {
        self.stack_push_16(self.regs.pc);
        let mut flag = self.regs.p;

        set_bit(&mut flag, CPUStatusFlags::BreakFlag as u8, false);
        set_bit(&mut flag, CPUStatusFlags::Break2Flag as u8, true);
 
        self.stack_push(flag);
        set_bit(&mut self.regs.p, CPUStatusFlags::InterruptDisable as u8, true);
 
        self.bus.tick(2);
        self.regs.pc = self.read_16(0xFFFA);
    }
}

impl Memory for CPU<'_> {
    fn read(&mut self, addr: u16) -> u8 {
        self.bus.read(addr)
    }
 
    fn write(&mut self, addr: u16, value: u8) {
        self.bus.write(addr, value)
    }

    /*
    NES CPU uses Little-Endian addressing rather than Big-Endian.
    That means that the 8 least significant bits of an address will be stored before the 8 most significant bits.
    */
    fn read_16(&mut self, addr: u16) -> u16 {
        self.bus.read_16(addr)
    }

    fn write_16(&mut self, addr: u16, value: u16) {
        self.bus.write_16(addr, value)
    }
}