use crate::memory::Memory;
use crate::registers::Registers;

#[derive(Default)]
pub(crate) struct CPU {
    pub regs: Registers,
    pub memory: Memory,
}


impl CPU {
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.memory.write_16(0xFFFC, 0x8000);
    }

    pub fn run(&mut self) {
        // note: we move  intialization of program_counter from here to load function
        loop {
            let opscode = self.memory.read(self.regs.pc);
            self.regs.pc += 1;

            let instruction = self.get_instruction(opscode);

            match opscode {
                0x00 => return,
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(&instruction.addr_mode),
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&instruction.addr_mode),
                0x0A => self.asl_accumulator(),
                0x06 | 0x16 | 0x0E | 0x1E => self.asl(&instruction.addr_mode),
                0x90 => self.bcc(),
                0xB0 => self.bcs(),
                0xF0 => self.beq(),
                0x24 | 0x2C => self.bit(&instruction.addr_mode),
                0x30 => self.beq(),
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
                0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&instruction.addr_mode),
                0xE8 => self.inx(),
                0xC8 => self.iny(),
                0x4C => self.jmp(),
                0x6C => self.jmp_indirect(),
                0x20 => self.jsr(),
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&instruction.addr_mode),
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&instruction.addr_mode),
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&instruction.addr_mode),

















                0x85 => {
                    self.sta(&AddressingMode::ZeroPage);
                    self.regs.pc += 1;
                },
                0x95 => {
                    self.sta(&AddressingMode::ZeroPageX);
                    self.regs.pc += 1;
                },
                // 0xA5 => {
                //     self.lda(&AddressingMode::ZeroPage);
                //     self.regs.pc += 1;
                // },
                // 0xA9 => {
                //     self.lda(&AddressingMode::Immediate);
                //     self.regs.pc += 1;
                // },
                0xAA => self.tax(),
                // 0xAD => {
                //     self.lda(&AddressingMode::Absolute);
                //     self.regs.pc += 2;
                // },
                // 0xE8 =>self.inx(),
                _ => todo!()
            }
        }
    }
        
    pub fn reset(&mut self) {
        self.regs.reset();
        self.regs.pc = self.memory.read_16(0xFFFC);
    }
}

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

impl CPU {
    pub(crate) fn get_op_addr(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.regs.pc,
            AddressingMode::ZeroPage  => self.memory.read(self.regs.pc) as u16,
            AddressingMode::Absolute => self.memory.read_16(self.regs.pc),
            AddressingMode::ZeroPageX => {
                let pos = self.memory.read(self.regs.pc);
                let addr = pos.wrapping_add(self.regs.x) as u16;
                addr
            },
            AddressingMode::ZeroPageY => {
                let pos = self.memory.read(self.regs.pc);
                let addr = pos.wrapping_add(self.regs.y) as u16;
                addr
            },
            AddressingMode::AbsoluteX => {
                let base = self.memory.read_16(self.regs.pc);
                let addr = base.wrapping_add(self.regs.x as u16);
                addr
            },
            AddressingMode::AbsoluteY => {
                let base = self.memory.read_16(self.regs.pc);
                let addr = base.wrapping_add(self.regs.y as u16);
                addr
            },
            AddressingMode::IndirectX => {
                let base = self.memory.read(self.regs.pc);
 
                let ptr: u8 = (base as u8).wrapping_add(self.regs.x);
                let lo = self.memory.read(ptr as u16);
                let hi = self.memory.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            },
            AddressingMode::IndirectY => {
                let base = self.memory.read(self.regs.pc);
 
                let lo = self.memory.read(base as u16);
                let hi = self.memory.read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.regs.y as u16);
                deref
            },
            AddressingMode::NoneAddressing => {
                panic!("Addressing mode {:?} is not supported", mode);
            }
        }
 
    }
}