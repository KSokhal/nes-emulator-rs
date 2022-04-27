use crate::bus::Memory;
use crate::lib::{get_bit, set_bit};
use crate::cpu::{CPU, AddressingMode};
use crate::registers::CPUStatusFlags;

const STACK: u16 = 0x0100;
pub const STACK_RESET: u8 = 0xFD;

pub struct Instruction {
    pub addr_mode: AddressingMode,
    pub name: &'static str,
    pub bytes: u8,
    pub cycles: u8,
}

impl CPU<'_> {
    pub(crate) fn get_instruction(&self, opcode: u8) -> Instruction {
        match opcode {
            0x69 => Instruction { addr_mode: AddressingMode::Immediate, name: "ADC", bytes: 2, cycles: 2 },
            0x65 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "ADC", bytes: 2, cycles: 3 },
            0x75 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "ADC", bytes: 2, cycles: 4 },
            0x6D => Instruction { addr_mode: AddressingMode::Absolute, name: "ADC", bytes: 3, cycles: 4 },
            0x7D => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "ADC", bytes: 3, cycles: 4 /* +1 if page crossed */ }, 
            0x79 => Instruction { addr_mode: AddressingMode::AbsoluteY, name: "ADC", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0x61 => Instruction { addr_mode: AddressingMode::IndirectX, name: "ADC", bytes: 2, cycles: 6 },
            0x71 => Instruction { addr_mode: AddressingMode::IndirectY, name: "ADC", bytes: 2, cycles: 5 /* +1 if page crossed */ },

            0x29 => Instruction { addr_mode: AddressingMode::Immediate, name: "AND", bytes: 2, cycles: 2 },
            0x25 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "AND", bytes: 2, cycles: 3 },
            0x35 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "AND", bytes: 2, cycles: 4 },
            0x2D => Instruction { addr_mode: AddressingMode::Absolute, name: "AND", bytes: 3, cycles: 4 },
            0x3D => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "AND", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0x39 => Instruction { addr_mode: AddressingMode::AbsoluteY, name: "AND", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0x21 => Instruction { addr_mode: AddressingMode::IndirectX, name: "AND", bytes: 2, cycles: 6 },
            0x31 => Instruction { addr_mode: AddressingMode::IndirectY, name: "AND", bytes: 2, cycles: 5 /* +1 if page crossed */ },

            0x0A => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "ASL_ACC", bytes: 1, cycles: 2 },
            0x06 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "ASL", bytes: 2, cycles: 5 },
            0x16 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "ASL", bytes: 2, cycles: 6 },
            0x0E => Instruction { addr_mode: AddressingMode::Absolute, name: "ASL", bytes: 3, cycles: 6 },
            0x1E => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "ASL", bytes: 3, cycles: 7 },

            0x90 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "BCC", bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },
            0xB0 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "BCS", bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },
            0xF0 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "BEQ", bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },
            
            0x24 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "BIT", bytes: 2, cycles: 3 },
            0x2C => Instruction { addr_mode: AddressingMode::Absolute, name: "BIT", bytes: 3, cycles: 4 },
        
            0x30 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "BMI", bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },
            0xD0 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "BNE", bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },
            0x10 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "BPL", bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },
            0x50 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "BVC", bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },
            0x70 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "BVS", bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },
        
            0x00 => Instruction {addr_mode: AddressingMode::NoneAddressing, name: "BRK", bytes: 1, cycles: 7},

            0x18 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "CLC", bytes: 1, cycles: 2 },
            0xD8 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "CLD", bytes: 1, cycles: 2 },
            0x58 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "CLI", bytes: 1, cycles: 2 },
            0xB8 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "CLV", bytes: 1, cycles: 2 },

            0xC9 => Instruction { addr_mode: AddressingMode::Immediate, name: "CMP", bytes: 2, cycles: 2 },
            0xC5 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "CMP", bytes: 2, cycles: 3 },
            0xD5 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "CMP", bytes: 2, cycles: 4 },
            0xCD => Instruction { addr_mode: AddressingMode::Absolute, name: "CMP", bytes: 3, cycles: 4 },
            0xDD => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "CMP", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0xD9 => Instruction { addr_mode: AddressingMode::AbsoluteY, name: "CMP", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0xC1 => Instruction { addr_mode: AddressingMode::IndirectX, name: "CMP", bytes: 2, cycles: 6 },
            0xD1 => Instruction { addr_mode: AddressingMode::IndirectY, name: "CMP", bytes: 2, cycles: 5 /* +1 if page crossed */ },

            0xE0 => Instruction { addr_mode: AddressingMode::Immediate, name: "CPX", bytes: 2, cycles: 2 },
            0xE4 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "CPX", bytes: 2, cycles: 3 },
            0xEC => Instruction { addr_mode: AddressingMode::Absolute, name: "CPX", bytes: 3, cycles: 4 },

            0xC0 => Instruction { addr_mode: AddressingMode::Immediate, name: "CPY", bytes: 2, cycles: 2 },
            0xC4 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "CPY", bytes: 2, cycles: 3 },
            0xCC => Instruction { addr_mode: AddressingMode::Absolute, name: "CPY", bytes: 3, cycles: 4 },
            
            0xC6 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "DEC", bytes: 2, cycles: 5 },
            0xD6 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "DEC", bytes: 2, cycles: 6 },
            0xCE => Instruction { addr_mode: AddressingMode::Absolute, name: "DEC", bytes: 3, cycles: 6 },
            0xDE => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "DEC", bytes: 3, cycles: 7 },
            
            0xCA => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "DEX", bytes: 1, cycles: 2 },
            0x88 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "DEY", bytes: 1, cycles: 2 },
            
            0x49 => Instruction { addr_mode: AddressingMode::Immediate, name: "EOR", bytes: 2, cycles: 2 },
            0x45 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "EOR", bytes: 2, cycles: 3 },
            0x55 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "EOR", bytes: 2, cycles: 4 },
            0x4D => Instruction { addr_mode: AddressingMode::Absolute, name: "EOR", bytes: 3, cycles: 4 },
            0x5D => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "EOR", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0x59 => Instruction { addr_mode: AddressingMode::AbsoluteY, name: "EOR", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0x41 => Instruction { addr_mode: AddressingMode::IndirectX, name: "EOR", bytes: 2, cycles: 6 },
            0x51 => Instruction { addr_mode: AddressingMode::IndirectY, name: "EOR", bytes: 2, cycles: 5 /* +1 if page crossed */ },
            
            0xE6 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "INC", bytes: 2, cycles: 5 },
            0xF6 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "INC", bytes: 2, cycles: 6 },
            0xEE => Instruction { addr_mode: AddressingMode::Absolute, name: "INC", bytes: 3, cycles: 6 },
            0xFE => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "INC", bytes: 3, cycles: 7 },
            
            0xE8 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "INX", bytes: 1, cycles: 2 },
            0xC8 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "INY", bytes: 1, cycles: 2 },
            
            0x4C => Instruction { addr_mode: AddressingMode::Absolute, name: "JMP", bytes: 3, cycles: 3 },
            0x6C => Instruction { addr_mode: AddressingMode::NoneAddressing , name: "JMP_INDIRECT", bytes: 3, cycles: 5 },
            
            0x20 => Instruction { addr_mode: AddressingMode::Absolute , name: "JSR", bytes: 3, cycles: 6 },
            
            0xA9 => Instruction { addr_mode: AddressingMode::Immediate, name: "LDA", bytes: 2, cycles: 2 },
            0xA5 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "LDA", bytes: 2, cycles: 3 },
            0xB5 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "LDA", bytes: 2, cycles: 4 },
            0xAD => Instruction { addr_mode: AddressingMode::Absolute, name: "LDA", bytes: 3, cycles: 4 },
            0xBD => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "LDA", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0xB9 => Instruction { addr_mode: AddressingMode::AbsoluteY, name: "LDA", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0xA1 => Instruction { addr_mode: AddressingMode::IndirectX, name: "LDA", bytes: 2, cycles: 6 },
            0xB1 => Instruction { addr_mode: AddressingMode::IndirectY, name: "LDA", bytes: 2, cycles: 5 /* +1 if page crossed */ },

            0xA2 => Instruction { addr_mode: AddressingMode::Immediate, name: "LDX", bytes: 2, cycles: 2 },
            0xA6 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "LDX", bytes: 2, cycles: 3 },
            0xB6 => Instruction { addr_mode: AddressingMode::ZeroPageY, name: "LDX", bytes: 2, cycles: 4 },
            0xAE => Instruction { addr_mode: AddressingMode::Absolute, name: "LDX", bytes: 3, cycles: 4 },
            0xBE => Instruction { addr_mode: AddressingMode::AbsoluteY, name: "LDX", bytes: 3, cycles: 4 /* +1 if page crossed */ },

            0xA0 => Instruction { addr_mode: AddressingMode::Immediate, name: "LDY", bytes: 2, cycles: 2 },
            0xA4 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "LDY", bytes: 2, cycles: 3 },
            0xB4 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "LDY", bytes: 2, cycles: 4 },
            0xAC => Instruction { addr_mode: AddressingMode::Absolute, name: "LDY", bytes: 3, cycles: 4 },
            0xBC => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "LDY", bytes: 3, cycles: 4 /* +1 if page crossed */ },

            0x4A => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "LSR_ACC", bytes: 1, cycles: 2 },
            0x46 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "LSR", bytes: 2, cycles: 5 },
            0x56 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "LSR", bytes: 2, cycles: 6 },
            0x4E => Instruction { addr_mode: AddressingMode::Absolute, name: "LSR", bytes: 3, cycles: 6 },
            0x5E => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "LSR", bytes: 3, cycles: 7 },

            0xEA => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "NOP", bytes: 1 , cycles: 2 },

            0x09 => Instruction { addr_mode: AddressingMode::Immediate, name: "ORA", bytes: 2, cycles: 2 },
            0x05 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "ORA", bytes: 2, cycles: 3 },
            0x15 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "ORA", bytes: 2, cycles: 4 },
            0x0D => Instruction { addr_mode: AddressingMode::Absolute, name: "ORA", bytes: 3, cycles: 4 },
            0x1D => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "ORA", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0x19 => Instruction { addr_mode: AddressingMode::AbsoluteY, name: "ORA", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0x01 => Instruction { addr_mode: AddressingMode::IndirectX, name: "ORA", bytes: 2, cycles: 6 },
            0x11 => Instruction { addr_mode: AddressingMode::IndirectY, name: "ORA", bytes: 2, cycles: 5 /* +1 if page crossed */ },

            0x48 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "PHA", bytes: 1, cycles: 3 },
            0x08 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "PHP", bytes: 1, cycles: 3 },
            0x68 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "PLA", bytes: 1, cycles: 4 },
            0x28 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "PLP", bytes: 1, cycles: 4 },
            
            0x2A => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "ROL", bytes: 1, cycles: 2 },
            0x26 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "ROL", bytes: 2, cycles: 5 },
            0x36 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "ROL", bytes: 2, cycles: 6 },
            0x2E => Instruction { addr_mode: AddressingMode::Absolute, name: "ROL", bytes: 3, cycles: 6 },
            0x3E => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "ROL", bytes: 3, cycles: 7 },

            0x6A => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "ROR", bytes: 1, cycles: 2 },
            0x66 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "ROR", bytes: 2, cycles: 5 },
            0x76 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "ROR", bytes: 2, cycles: 6 },
            0x6E => Instruction { addr_mode: AddressingMode::Absolute, name: "ROR", bytes: 3, cycles: 6 },
            0x7E => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "ROR", bytes: 3, cycles: 7 },

            0x40 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "RTI", bytes: 1, cycles: 6 },
            0x60 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "RTS", bytes: 1, cycles: 6 },

            0xE9 => Instruction { addr_mode: AddressingMode::Immediate, name: "SBC", bytes: 2, cycles: 2 },
            0xE5 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "SBC", bytes: 2, cycles: 3 },
            0xF5 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "SBC", bytes: 2, cycles: 4 },
            0xED => Instruction { addr_mode: AddressingMode::Absolute, name: "SBC", bytes: 3, cycles: 4 },
            0xFD => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "SBC", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0xF9 => Instruction { addr_mode: AddressingMode::AbsoluteY, name: "SBC", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            0xE1 => Instruction { addr_mode: AddressingMode::IndirectX, name: "SBC", bytes: 2, cycles: 6 },
            0xF1 => Instruction { addr_mode: AddressingMode::IndirectY, name: "SBC", bytes: 2, cycles: 5 /* +1 if page crossed */ },

            0x38 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "SEC", bytes: 1, cycles: 2 },
            0xF8 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "SED", bytes: 1, cycles: 2 },
            0x78 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "SEI", bytes: 1, cycles: 2 },

            0x85 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "STA", bytes: 2, cycles: 3 },
            0x95 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "STA", bytes: 2, cycles: 4 },
            0x8D => Instruction { addr_mode: AddressingMode::Absolute, name: "STA", bytes: 3, cycles: 4 },
            0x9D => Instruction { addr_mode: AddressingMode::AbsoluteX, name: "STA", bytes: 3, cycles: 5 },
            0x99 => Instruction { addr_mode: AddressingMode::AbsoluteY, name: "STA", bytes: 3, cycles: 5 },
            0x81 => Instruction { addr_mode: AddressingMode::IndirectX, name: "STA", bytes: 2, cycles: 6 },
            0x91 => Instruction { addr_mode: AddressingMode::IndirectY, name: "STA", bytes: 2, cycles: 6 },

            0x86 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "STX", bytes: 2, cycles: 3 },
            0x96 => Instruction { addr_mode: AddressingMode::ZeroPageY, name: "STX", bytes: 2, cycles: 4 },
            0x8E => Instruction { addr_mode: AddressingMode::Absolute, name: "STX", bytes: 3, cycles: 4 },

            0x84 => Instruction { addr_mode: AddressingMode::ZeroPage, name: "STY", bytes: 2, cycles: 3 },
            0x94 => Instruction { addr_mode: AddressingMode::ZeroPageX, name: "STY", bytes: 2, cycles: 4 },
            0x8C => Instruction { addr_mode: AddressingMode::Absolute, name: "STY", bytes: 3, cycles: 4 },

            0xAA => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "TAX", bytes: 1, cycles: 2 },
            0xA8 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "TAY", bytes: 1, cycles: 2 },
            0xBA => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "TSX", bytes: 1, cycles: 2 },
            0x8A => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "TXA", bytes: 1, cycles: 2 },
            0x9A => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "TXS", bytes: 1, cycles: 2 },
            0x98 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "TYA", bytes: 1, cycles: 2 },
            
            // Unofficial Opcodes

            0xc7 => Instruction { name: "*DCP", bytes: 2, cycles: 5, addr_mode: AddressingMode::ZeroPage },
            0xd7 => Instruction { name: "*DCP", bytes: 2, cycles: 6, addr_mode: AddressingMode::ZeroPageX },
            0xCF => Instruction { name: "*DCP", bytes: 3, cycles: 6, addr_mode: AddressingMode::Absolute },
            0xDF => Instruction { name: "*DCP", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteX },
            0xDB => Instruction { name: "*DCP", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteY },
            0xd3 => Instruction { name: "*DCP", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectY },
            0xc3 => Instruction { name: "*DCP", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectX },


            0x27 => Instruction { name: "*RLA", bytes: 2, cycles: 5, addr_mode: AddressingMode::ZeroPage },
            0x37 => Instruction { name: "*RLA", bytes: 2, cycles: 6, addr_mode: AddressingMode::ZeroPageX },
            0x2F => Instruction { name: "*RLA", bytes: 3, cycles: 6, addr_mode: AddressingMode::Absolute },
            0x3F => Instruction { name: "*RLA", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteX },
            0x3b => Instruction { name: "*RLA", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteY },
            0x33 => Instruction { name: "*RLA", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectY },
            0x23 => Instruction { name: "*RLA", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectX },

            0x07 => Instruction { name: "*SLO", bytes: 2, cycles: 5, addr_mode: AddressingMode::ZeroPage },
            0x17 => Instruction { name: "*SLO", bytes: 2, cycles: 6, addr_mode: AddressingMode::ZeroPageX },
            0x0F => Instruction { name: "*SLO", bytes: 3, cycles: 6, addr_mode: AddressingMode::Absolute },
            0x1f => Instruction { name: "*SLO", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteX },
            0x1b => Instruction { name: "*SLO", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteY },
            0x03 => Instruction { name: "*SLO", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectX },
            0x13 => Instruction { name: "*SLO", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectY },

            0x47 => Instruction { name: "*SRE", bytes: 2, cycles: 5, addr_mode: AddressingMode::ZeroPage },
            0x57 => Instruction { name: "*SRE", bytes: 2, cycles: 6, addr_mode: AddressingMode::ZeroPageX },
            0x4F => Instruction { name: "*SRE", bytes: 3, cycles: 6, addr_mode: AddressingMode::Absolute },
            0x5f => Instruction { name: "*SRE", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteX },
            0x5b => Instruction { name: "*SRE", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteY },
            0x43 => Instruction { name: "*SRE", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectX },
            0x53 => Instruction { name: "*SRE", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectY },


            0x80 => Instruction { name: "*NOP", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },
            0x82 => Instruction { name: "*NOP", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },
            0x89 => Instruction { name: "*NOP", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },
            0xc2 => Instruction { name: "*NOP", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },
            0xe2 => Instruction { name: "*NOP", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },


            0xCB => Instruction { name: "*AXS", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },

            0x6B => Instruction { name: "*ARR", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },

            0xeb => Instruction { name: "*SBC", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },

            0x0b => Instruction { name: "*ANC", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },
            0x2b => Instruction { name: "*ANC", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },

            0x4b => Instruction { name: "*ALR", bytes: 2, cycles: 2, addr_mode: AddressingMode::Immediate },
            // 0xCB => Instruction { name: "IGN", 3,4 /* or 5*/, AddressingMode::AbsoluteX),

            0x04 => Instruction { name: "*NOP", bytes: 2, cycles: 3, addr_mode: AddressingMode::ZeroPage },
            0x44 => Instruction { name: "*NOP", bytes: 2, cycles: 3, addr_mode: AddressingMode::ZeroPage },
            0x64 => Instruction { name: "*NOP", bytes: 2, cycles: 3, addr_mode: AddressingMode::ZeroPage },
            0x14 => Instruction { name: "*NOP", bytes: 2, cycles: 4, addr_mode: AddressingMode::ZeroPageX },
            0x34 => Instruction { name: "*NOP", bytes: 2, cycles: 4, addr_mode: AddressingMode::ZeroPageX },
            0x54 => Instruction { name: "*NOP", bytes: 2, cycles: 4, addr_mode: AddressingMode::ZeroPageX },
            0x74 => Instruction { name: "*NOP", bytes: 2, cycles: 4, addr_mode: AddressingMode::ZeroPageX },
            0xd4 => Instruction { name: "*NOP", bytes: 2, cycles: 4, addr_mode: AddressingMode::ZeroPageX },
            0xf4 => Instruction { name: "*NOP", bytes: 2, cycles: 4, addr_mode: AddressingMode::ZeroPageX },
            0x0c => Instruction { name: "*NOP", bytes: 3, cycles: 4, addr_mode: AddressingMode::Absolute },
            0x1c => Instruction { name: "*NOP", bytes: 3, cycles: 4 /*or 5*/, addr_mode: AddressingMode::AbsoluteX },
            0x3c => Instruction { name: "*NOP", bytes: 3, cycles: 4 /*or 5*/, addr_mode: AddressingMode::AbsoluteX },
            0x5c => Instruction { name: "*NOP", bytes: 3, cycles: 4 /*or 5*/, addr_mode: AddressingMode::AbsoluteX },
            0x7c => Instruction { name: "*NOP", bytes: 3, cycles: 4 /*or 5*/, addr_mode: AddressingMode::AbsoluteX },
            0xdc => Instruction { name: "*NOP", bytes: 3, cycles: 4 /* or 5*/, addr_mode: AddressingMode::AbsoluteX },
            0xfc => Instruction { name: "*NOP", bytes: 3, cycles: 4 /* or 5*/, addr_mode: AddressingMode::AbsoluteX },

            0x67 => Instruction { name: "*RRA", bytes: 2, cycles: 5, addr_mode: AddressingMode::ZeroPage },
            0x77 => Instruction { name: "*RRA", bytes: 2, cycles: 6, addr_mode: AddressingMode::ZeroPageX },
            0x6f => Instruction { name: "*RRA", bytes: 3, cycles: 6, addr_mode: AddressingMode::Absolute },
            0x7f => Instruction { name: "*RRA", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteX },
            0x7b => Instruction { name: "*RRA", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteY },
            0x63 => Instruction { name: "*RRA", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectX },
            0x73 => Instruction { name: "*RRA", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectY },


            0xe7 => Instruction { name: "*ISB", bytes: 2, cycles: 5, addr_mode: AddressingMode::ZeroPage },
            0xf7 => Instruction { name: "*ISB", bytes: 2, cycles: 6, addr_mode: AddressingMode::ZeroPageX },
            0xef => Instruction { name: "*ISB", bytes: 3, cycles: 6, addr_mode: AddressingMode::Absolute },
            0xff => Instruction { name: "*ISB", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteX },
            0xfb => Instruction { name: "*ISB", bytes: 3, cycles: 7, addr_mode: AddressingMode::AbsoluteY },
            0xe3 => Instruction { name: "*ISB", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectX },
            0xf3 => Instruction { name: "*ISB", bytes: 2, cycles: 8, addr_mode: AddressingMode::IndirectY },

            0x12 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x22 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x02 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x32 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x42 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x52 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x62 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x72 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x92 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0xb2 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0xd2 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0xf2 => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },

            0x1a => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x3a => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x5a => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0x7a => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            0xda => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },
            // 0xea => Instruction { name: "NOP", 1,2, AddressingMode::NoneAddressing),
            0xfa => Instruction { name: "*NOP", bytes: 1, cycles: 2, addr_mode: AddressingMode::NoneAddressing },

            0xab => Instruction { name: "*LXA", bytes: 2, cycles: 3, addr_mode: AddressingMode::Immediate }, //todo: highly unstable and not used
            //http://visual6502.org/wiki/index.php?title=6502_Opcode_8B_%28XAA,_ANE%29
            0x8b => Instruction { name: "*XAA", bytes: 2, cycles: 3, addr_mode: AddressingMode::Immediate }, //todo: highly unstable and not used
            0xbb => Instruction { name: "*LAS", bytes: 3, cycles: 2, addr_mode: AddressingMode::AbsoluteY }, //todo: highly unstable and not used
            0x9b => Instruction { name: "*TAS", bytes: 3, cycles: 2, addr_mode: AddressingMode::AbsoluteY }, //todo: highly unstable and not used
            0x93 => Instruction { name: "*AHX", bytes: 2, cycles: 8 /* guess */, addr_mode: AddressingMode::IndirectY }, //todo: highly unstable and not used
            0x9f => Instruction { name: "*AHX", bytes: 3, cycles: 4 /* or 5*/ /* guess */, addr_mode: AddressingMode::AbsoluteY }, //todo: highly unstable and not used
            0x9e => Instruction { name: "*SHX", bytes: 3, cycles: 4 /* or 5*/ /* guess */, addr_mode: AddressingMode::AbsoluteY }, //todo: highly unstable and not used
            0x9c => Instruction { name: "*SHY", bytes: 3, cycles: 4 /* or 5*/ /* guess */, addr_mode: AddressingMode::AbsoluteX }, //todo: highly unstable and not used

            0xa7 => Instruction { name: "*LAX", bytes: 2, cycles: 3, addr_mode: AddressingMode::ZeroPage },
            0xb7 => Instruction { name: "*LAX", bytes: 2, cycles: 4, addr_mode: AddressingMode::ZeroPageY },
            0xaf => Instruction { name: "*LAX", bytes: 3, cycles: 4, addr_mode: AddressingMode::Absolute },
            0xbf => Instruction { name: "*LAX", bytes: 3, cycles: 4, addr_mode: AddressingMode::AbsoluteY },
            0xa3 => Instruction { name: "*LAX", bytes: 2, cycles: 6, addr_mode: AddressingMode::IndirectX },
            0xb3 => Instruction { name: "*LAX", bytes: 2, cycles: 5, addr_mode: AddressingMode::IndirectY },

            0x87 => Instruction { name: "*SAX", bytes: 2, cycles: 3, addr_mode: AddressingMode::ZeroPage },
            0x97 => Instruction { name: "*SAX", bytes: 2, cycles: 4, addr_mode: AddressingMode::ZeroPageY },
            0x8f => Instruction { name: "*SAX", bytes: 3, cycles: 4, addr_mode: AddressingMode::Absolute },
            0x83 => Instruction { name: "*SAX", bytes: 2, cycles: 6, addr_mode: AddressingMode::IndirectX },
        }
    }

    fn update_result_flags(&mut self, result: u8) {
        set_bit(&mut self.regs.p, CPUStatusFlags::ZeroResult as u8, result == 0);
        set_bit(&mut self.regs.p, CPUStatusFlags::NegativeResult as u8, get_bit(result, 7));
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            self.bus.tick(1);

            let jump: i8 = self.read(self.regs.pc) as i8;
            let jump_addr = self.regs.pc.wrapping_add(1).wrapping_add(jump as u16);

            if self.regs.pc.wrapping_add(1) & 0xFF00 != jump_addr & 0xFF00 {
                self.bus.tick(1);
            }

            self.regs.pc = jump_addr;
        }
    }

    pub(crate) fn add_to_accumulator(&mut self, value: u8) {
        let (sum, did_overflow1) = self.regs.a.overflowing_add(value);
        let (accumulator, did_overflow2) = sum.overflowing_add(
            if get_bit(self.regs.p, CPUStatusFlags::CarryFlag as u8) {
                1
            } else {
                0
            }
        );
        
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, did_overflow1 | did_overflow2);
        set_bit(&mut self.regs.p, CPUStatusFlags::OverflowFlag as u8, (value ^ accumulator) & (accumulator ^ self.regs.a) & 0x80 != 0);
        
        self.regs.a = accumulator;
        self.update_result_flags(self.regs.a);
    }

    fn stack_pop(&mut self) -> u8 {
        self.regs.sp = self.regs.sp.wrapping_add(1);
        self.read((STACK as u16) + self.regs.sp as u16)
    }

    pub fn stack_push(&mut self, data: u8) {
        self.write((STACK as u16) + self.regs.sp as u16, data);
        self.regs.sp = self.regs.sp.wrapping_sub(1)
    }

    pub fn stack_push_16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pop_16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;

        hi << 8 | lo
    }

    pub(crate) fn adc(&mut self, mode: &AddressingMode) {
        let (addr, page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        if page_crossed {
            self.bus.tick(1);
        }

        self.add_to_accumulator(value);
    }

    pub(crate) fn and(&mut self, mode: &AddressingMode) {
        let (addr, page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        if page_crossed {
            self.bus.tick(1);
        }

        self.regs.a &= value;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn asl_accumulator(&mut self) {
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, get_bit(self.regs.a, 7));
        self.regs.a <<= 1;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn asl(&mut self, mode: &AddressingMode) -> u8 {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let mut value = self.read(addr);

        let bit = get_bit(value, 7);
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, bit);

        value <<= 1;
        self.write(addr, value);
        self.update_result_flags(value);
        value
    }

    pub(crate) fn bcc(&mut self) {
        self.branch(!get_bit(self.regs.p, CPUStatusFlags::CarryFlag as u8));
    }

    pub(crate) fn bcs(&mut self) {
        self.branch(get_bit(self.regs.p, CPUStatusFlags::CarryFlag as u8));
    }

    pub(crate) fn beq(&mut self) {
        self.branch(get_bit(self.regs.p, CPUStatusFlags::ZeroResult as u8));
    }

    pub(crate) fn bit(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);
        
        let check = self.regs.a & value;
        set_bit(&mut self.regs.p, CPUStatusFlags::ZeroResult as u8, check == 0);
        set_bit(&mut self.regs.p, CPUStatusFlags::NegativeResult as u8, get_bit(value, 7));
        set_bit(&mut self.regs.p, CPUStatusFlags::OverflowFlag as u8, get_bit(value, 6));
    }

    pub(crate) fn bmi(&mut self) {
        self.branch(get_bit(self.regs.p, CPUStatusFlags::NegativeResult as u8));
    }

    pub(crate) fn bne(&mut self) {
        self.branch(!get_bit(self.regs.p, CPUStatusFlags::ZeroResult as u8));
    }

    pub(crate) fn bpl(&mut self) {
        self.branch(!get_bit(self.regs.p, CPUStatusFlags::NegativeResult as u8));
    }

    pub(crate) fn bvc(&mut self) {
        self.branch(!get_bit(self.regs.p, CPUStatusFlags::OverflowFlag as u8));
    }

    pub(crate) fn bvs(&mut self) {
        self.branch(get_bit(self.regs.p, CPUStatusFlags::OverflowFlag as u8));
    }

    pub(crate) fn clc(&mut self) {
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, false);
    }

    pub(crate) fn cld(&mut self) {
        set_bit(&mut self.regs.p, CPUStatusFlags::DecimalMode as u8, false);
    }

    pub(crate) fn cli(&mut self) {
        set_bit(&mut self.regs.p, CPUStatusFlags::InterruptDisable as u8, false);
    }
    
    pub(crate) fn clv(&mut self) {
        set_bit(&mut self.regs.p, CPUStatusFlags::OverflowFlag as u8, false);
    }

    pub(crate) fn compare(&mut self, mode: &AddressingMode, compare_reg: u8) {
        let (addr, page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        if page_crossed {
            self.bus.tick(1);
        };

        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, compare_reg >= value);
        let result = compare_reg.wrapping_sub(value);
        self.update_result_flags(result);
    }
 
    pub(crate) fn dec(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);
        let result = value.wrapping_sub(1);

        self.write(addr, result);
        
        self.update_result_flags(result);
    }

    pub(crate) fn dex(&mut self) {
        self.regs.x = self.regs.x.wrapping_sub(1);

        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn dey(&mut self) {
        self.regs.y = self.regs.y.wrapping_sub(1);

        self.update_result_flags(self.regs.y);
    }

    pub(crate) fn eor(&mut self, mode: &AddressingMode) {
        let (addr, page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        if page_crossed {
            self.bus.tick(1);
        };

        self.regs.a ^= value;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn inc(&mut self, mode: &AddressingMode) -> u8 {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);
        let result = value.wrapping_add(1);

        self.update_result_flags(result);

        self.write(addr, result);
        result
    }

    pub(crate) fn inx(&mut self) {
        self.regs.x = self.regs.x.wrapping_add(1);

        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn iny(&mut self) {
        self.regs.y = self.regs.y.wrapping_add(1);

        self.update_result_flags(self.regs.y);
    }

    pub(crate) fn jmp(&mut self) {
        let mem_address = self.read_16(self.regs.pc);
        self.regs.pc = mem_address;
    }

    pub(crate) fn jmp_indirect(&mut self) {
        let mem_address = self.read_16(self.regs.pc);

        let indirect_ref = if mem_address & 0x00FF == 0x00FF {
            let lo = self.read(mem_address);
            let hi = self.read(mem_address & 0xFF00);
            (hi as u16) << 8 | (lo as u16)
        } else {
            self.read_16(mem_address)
        };

        self.regs.pc = indirect_ref;
    }

    pub(crate) fn jsr(&mut self) {
        self.stack_push_16(self.regs.pc + 2 - 1);
        let target_address = self.read_16(self.regs.pc);
        self.regs.pc = target_address
    }

    pub(crate) fn lda(&mut self, mode: &AddressingMode) {
        let (addr, page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        if page_crossed {
            self.bus.tick(1);
        };

        self.regs.a = value;
        self.update_result_flags(self.regs.a);
    }
     
    pub(crate) fn ldx(&mut self, mode: &AddressingMode) {
        let (addr, page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        if page_crossed {
            self.bus.tick(1);
        };

        self.regs.x = value;
        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn ldy(&mut self, mode: &AddressingMode) {
        let (addr, page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        if page_crossed {
            self.bus.tick(1);
        };

        self.regs.y = value;
        self.update_result_flags(self.regs.y);
    }

    pub(crate) fn lsr_accumulator(&mut self) {
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, get_bit(self.regs.a, 0));
        self.regs.a >>= 1;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn lsr(&mut self, mode: &AddressingMode) -> u8 {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let mut value = self.read(addr);

        let bit = get_bit(value, 0);
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, bit);

        value >>= 1;
        self.write(addr, value);
        self.update_result_flags(value);
        value
    }

    pub(crate) fn ora(&mut self, mode: &AddressingMode) {
        let (addr, page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        if page_crossed {
            self.bus.tick(1);
        };

        self.regs.a |= value;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn pha(&mut self) {
        self.stack_push(self.regs.a);
    }

    pub(crate) fn php(&mut self) {
        self.stack_push(self.regs.p);
    }

    pub(crate) fn pla(&mut self) {
        self.regs.a = self.stack_pop();
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn plp(&mut self) {
        self.regs.p = self.stack_pop();
    }

    pub(crate) fn rol_accumulator(&mut self) {
        let old_carry = get_bit(self.regs.p, CPUStatusFlags::CarryFlag as u8); 
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, get_bit(self.regs.a, 7));
        self.regs.a <<= 1;
        set_bit(&mut self.regs.a, 0, old_carry);
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn rol(&mut self, mode: &AddressingMode) -> u8 {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let mut value = self.read(addr);

        let old_carry = get_bit(self.regs.p, CPUStatusFlags::CarryFlag as u8); 
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, get_bit(value, 7));
        value <<= 1;
        set_bit(&mut value, 0, old_carry);

        self.write(addr, value);
        self.update_result_flags(value);
        value
    }

    pub(crate) fn ror_accumulator(&mut self) {
        let old_carry = get_bit(self.regs.p, CPUStatusFlags::CarryFlag as u8); 
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, get_bit(self.regs.a, 0));
        self.regs.a >>= 1;
        set_bit(&mut self.regs.a, 7, old_carry);
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn ror(&mut self, mode: &AddressingMode) -> u8 {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let mut value = self.read(addr);

        let old_carry = get_bit(self.regs.p, CPUStatusFlags::CarryFlag as u8); 
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, get_bit(value, 0));
        value >>= 1;
        set_bit(&mut value, 7, old_carry);

        self.write(addr, value);
        self.update_result_flags(value);
        value
    }

    pub(crate) fn rti(&mut self) {
        self.regs.p = self.stack_pop();
        self.regs.pc = self.stack_pop_16();
    }

    pub(crate) fn rts(&mut self) {
        self.regs.pc = self.stack_pop_16() + 1;
    }

    pub(crate) fn sbc(&mut self, mode: &AddressingMode) {
        let (addr, page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        if page_crossed {
            self.bus.tick(1);
        };

        // self.sub_from_accumulator(value);
        self.add_to_accumulator(((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }

    pub(crate) fn sta(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        self.write(addr, self.regs.a);
    }

    pub(crate) fn stx(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        self.write(addr, self.regs.x);
    }

    pub(crate) fn sty(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        self.write(addr, self.regs.y);
    }

    pub(crate) fn tax(&mut self) {
        self.regs.x = self.regs.a;
        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn tay(&mut self) {
        self.regs.y = self.regs.a;
        self.update_result_flags(self.regs.y);
    }

    pub(crate) fn tsx(&mut self) {
        self.regs.x = self.regs.sp;
        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn txa(&mut self) {
        self.regs.a = self.regs.x;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn txs(&mut self) {
        self.regs.sp = self.regs.x;
    }

    pub(crate) fn tya(&mut self) {
        self.regs.a = self.regs.y;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn dcp(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let mut value = self.read(addr);

        value = value.wrapping_sub(1);
        self.write(addr, value);

        if value <= self.regs.a {
            set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, true)
        }

        self.update_result_flags(self.regs.a.wrapping_sub(value));
    }

    pub(crate) fn rla(&mut self, mode: &AddressingMode) {
        let data = self.rol(mode);
        self.regs.a &= data;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn slo(&mut self, mode: &AddressingMode) {
        let data = self.asl(mode);
        self.regs.a |= data;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn sre(&mut self, mode: &AddressingMode) {
        let data = self.lsr(mode);
        self.regs.a ^= data;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn axs(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        let x_and_a = self.regs.x & self.regs.a;
        let result = x_and_a.wrapping_sub(value);

        if value <= x_and_a {
            set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, true);
        }

        self.update_result_flags(result);

        self.regs.x = result;
    }

    pub(crate) fn arr(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        self.regs.a &= value;
        self.update_result_flags(self.regs.a);

        self.ror_accumulator();

        let result = self.regs.a;
        let bit_5 = get_bit(result, 5);
        let bit_6 = get_bit(result, 6);

        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, bit_6);

        set_bit(&mut self.regs.p, CPUStatusFlags::OverflowFlag as u8, bit_5 ^ bit_6);

        self.update_result_flags(result);
    }

    pub(crate) fn sbc_unofficial(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        self.add_to_accumulator(((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }

    pub(crate) fn anc(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        self.regs.a &= value;
        self.update_result_flags(self.regs.a);

        let negative_flag = get_bit(self.regs.p, CPUStatusFlags::NegativeResult as u8);
        set_bit(&mut self.regs.p, CPUStatusFlags::CarryFlag as u8, negative_flag);
    }

    pub(crate) fn alr(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        self.regs.a &= value;
        self.update_result_flags(self.regs.a);

        self.lsr_accumulator();
    }

    pub(crate) fn rra(&mut self, mode: &AddressingMode) {
        let value = self.ror(mode);

        self.add_to_accumulator(value);
    }

    pub(crate) fn isb(&mut self, mode: &AddressingMode) {
        let value = self.inc(mode);

        self.add_to_accumulator(((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }

    pub(crate) fn lax(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        self.regs.a = value;
        self.update_result_flags(self.regs.a);
        self.regs.x = self.regs.a;
    }

    pub(crate) fn sax(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.regs.a & self.regs.x;

        self.write(addr, value);
    }

    pub(crate) fn lxa(&mut self, mode: &AddressingMode) {
        self.lda(mode);
        self.tax();
    }

    pub(crate) fn xaa(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        self.regs.a = self.regs.x;
        self.update_result_flags(self.regs.a);
        
        self.regs.a &= value;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn las(&mut self, mode: &AddressingMode) {
        let (addr, _page_crossed) = self.get_op_addr(mode);
        let value = self.read(addr);

        let result = value & self.regs.sp;

        self.regs.a = result;
        self.regs.x = result;
        self.regs.sp = result;
        self.update_result_flags(result);
    }

    pub(crate) fn tas(&mut self) {
        let value = self.regs.a & self.regs.x;
        self.regs.sp = value;

        let mem_address = self.read_16(self.regs.pc) + self.regs.y as u16;

        let data = ((mem_address >> 8) as u8 + 1) & self.regs.sp;
        self.write(mem_address, data);
    }

    pub(crate) fn ahx_indir_y(&mut self) {
        let pos: u8 = self.read(self.regs.pc);
        let mem_address = self.read_16(pos as u16) + self.regs.y as u16;

        let data = self.regs.a & self.regs.x & (mem_address >> 8) as u8;
        self.write(mem_address, data);
    }

    pub(crate) fn ahx_abs_y(&mut self) {
        let mem_address = self.read_16(self.regs.pc) + self.regs.y as u16;

        let data = self.regs.a & self.regs.x & (mem_address >> 8) as u8;
        self.write(mem_address, data);
    }

    pub(crate) fn shx(&mut self) {
        let mem_address = self.read_16(self.regs.pc) + self.regs.y as u16;

        // todo if cross page boundry {
        //     mem_address &= (self.x as u16) << 8;
        // }
        let data = self.regs.x & ((mem_address >> 8) as u8 + 1);
        self.write(mem_address, data);
    }

    pub(crate) fn shy(&mut self) {
        let mem_address = self.read_16(self.regs.pc) + self.regs.x as u16;
        let data = self.regs.y & ((mem_address >> 8) as u8 + 1);
        self.write(mem_address, data)
    }
}