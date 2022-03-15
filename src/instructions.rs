use crate::lib::{get_bit, set_bit};
use crate::cpu::{CPU, AddressingMode};
use crate::registers::{ZERO_RESULT_FLAG_BYTE_POSITION, NEGATIVE_RESULT_FLAG_BYTE_POSITION, CARRY_FLAG_BYTE_POSITION, OVERFLOW_FLAG_BYTE_POSITION, DECIMAL_MODE_FLAG_BYTE_POSITION, INTERRUPT_DISABLE_FLAG_BYTE_POSITION};

pub struct Instruction {
    pub addr_mode: AddressingMode,
    name: &'static str,
    bytes: u8,
    cycles: u8,
}

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;

impl CPU {
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
            0x70 => Instruction { addr_mode: AddressingMode::NoneAddressing, name: "BPL", bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },
        
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








            
            
            
            _ => panic!("Intruction not implemented for opcode: {:?}", opcode)
        }
    }

    fn update_result_flags(&mut self, result: u8) {
        set_bit(&mut self.regs.p, ZERO_RESULT_FLAG_BYTE_POSITION, result == 0);
        set_bit(&mut self.regs.p, NEGATIVE_RESULT_FLAG_BYTE_POSITION, get_bit(result, 7));
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump: i8 = self.memory.read(self.regs.pc) as i8;
            let jump_addr = self.regs.pc.wrapping_add(1).wrapping_add(jump as u16);

            self.regs.pc = jump_addr;
        }
    }

    pub(crate) fn add_to_accumulator(&mut self, value: u8) {
        let (sum, did_overflow1) = self.regs.a.overflowing_add(value);
        let (accumulator, did_overflow2) = sum.overflowing_add(
            if get_bit(self.regs.p, CARRY_FLAG_BYTE_POSITION) {
                1
            } else {
                0
            }
        );
        
        set_bit(&mut self.regs.p, CARRY_FLAG_BYTE_POSITION, did_overflow1 | did_overflow2);
        set_bit(&mut self.regs.p, OVERFLOW_FLAG_BYTE_POSITION, (value ^ accumulator) & (accumulator ^ self.regs.a) & 0x80 != 0);
        
        self.regs.a = accumulator;
    }


    fn stack_pop(&mut self) -> u8 {
        self.regs.sp = self.regs.sp.wrapping_add(1);
        self.memory.read((STACK as u16) + self.regs.sp as u16)
    }

    fn stack_push(&mut self, data: u8) {
        self.memory.write((STACK as u16) + self.regs.sp as u16, data);
        self.regs.sp = self.regs.sp.wrapping_sub(1)
    }

    fn stack_push_16(&mut self, data: u16) {
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
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);

        self.add_to_accumulator(value);
    }

    pub(crate) fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);

        self.regs.a &= value;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn asl_accumulator(&mut self) {
        set_bit(&mut self.regs.p, CARRY_FLAG_BYTE_POSITION, get_bit(self.regs.a, 7));
        self.regs.a <<= 1;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let mut value = self.memory.read(addr);

        let bit = get_bit(value, 7);
        set_bit(&mut value, CARRY_FLAG_BYTE_POSITION, bit);

        value <<= 1;
        self.memory.write(addr, value);
        self.update_result_flags(value);
    }

    pub(crate) fn bcc(&mut self) {
        self.branch(!get_bit(self.regs.p, CARRY_FLAG_BYTE_POSITION));
    }

    pub(crate) fn bcs(&mut self) {
        self.branch(get_bit(self.regs.p, CARRY_FLAG_BYTE_POSITION));
    }

    pub(crate) fn beq(&mut self) {
        self.branch(get_bit(self.regs.p, ZERO_RESULT_FLAG_BYTE_POSITION));
    }

    pub(crate) fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);
        
        let check = self.regs.a & value;
        set_bit(&mut self.regs.p, OVERFLOW_FLAG_BYTE_POSITION, get_bit(check, 6));
        self.update_result_flags(check);
    }

    pub(crate) fn bne(&mut self) {
        self.branch(!get_bit(self.regs.p, ZERO_RESULT_FLAG_BYTE_POSITION));
    }

    pub(crate) fn bpl(&mut self) {
        self.branch(!get_bit(self.regs.p, NEGATIVE_RESULT_FLAG_BYTE_POSITION));
    }

    pub(crate) fn bvc(&mut self) {
        self.branch(!get_bit(self.regs.p, OVERFLOW_FLAG_BYTE_POSITION));
    }

    pub(crate) fn bvs(&mut self) {
        self.branch(get_bit(self.regs.p, OVERFLOW_FLAG_BYTE_POSITION));
    }

    pub(crate) fn clc(&mut self) {
        set_bit(&mut self.regs.p, CARRY_FLAG_BYTE_POSITION, false);
    }

    pub(crate) fn cld(&mut self) {
        set_bit(&mut self.regs.p, DECIMAL_MODE_FLAG_BYTE_POSITION, false);
    }

    pub(crate) fn cli(&mut self) {
        set_bit(&mut self.regs.p, INTERRUPT_DISABLE_FLAG_BYTE_POSITION, false);
    }
    
    pub(crate) fn clv(&mut self) {
        set_bit(&mut self.regs.p, OVERFLOW_FLAG_BYTE_POSITION, false);
    }

    pub(crate) fn compare(&mut self, mode: &AddressingMode, compare_reg: u8) {
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);

        set_bit(&mut self.regs.p, CARRY_FLAG_BYTE_POSITION, compare_reg >= value);
        let result = compare_reg.wrapping_sub(value);
        self.update_result_flags(result);
    }
 
    pub(crate) fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);
        let result = value.wrapping_sub(1);

        self.update_result_flags(result);

        self.memory.write(addr, result)
    }

    pub(crate) fn dex(&mut self) {
        self.regs.x = self.regs.x.wrapping_sub(1);

        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn dey(&mut self) {
        self.regs.x = self.regs.x.wrapping_sub(1);

        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);

        self.regs.a ^= value;
        self.update_result_flags(self.regs.a);
    }

    pub(crate) fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);
        let result = value.wrapping_add(1);

        self.update_result_flags(result);

        self.memory.write(addr, result)
    }

    pub(crate) fn inx(&mut self) {
        self.regs.x = self.regs.x.wrapping_add(1);

        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn iny(&mut self) {
        self.regs.x = self.regs.x.wrapping_add(1);

        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn jmp(&mut self) {
        let mem_address = self.memory.read_16(self.regs.pc);
        self.regs.pc = mem_address;
    }

    pub(crate) fn jmp_indirect(&mut self) {
        let mem_address = self.memory.read_16(self.regs.pc);

        let indirect_ref = if mem_address & 0x00FF == 0x00FF {
            let lo = self.memory.read(mem_address);
            let hi = self.memory.read(mem_address & 0xFF00);
            (hi as u16) << 8 | (lo as u16)
        } else {
            self.memory.read_16(mem_address)
        };

        self.regs.pc = indirect_ref;
    }

    pub(crate) fn jsr(&mut self) {
        self.stack_push_16(self.regs.pc + 2 - 1);
        let target_address = self.memory.read_16(self.regs.pc);
        self.regs.pc = target_address
    }

    pub(crate) fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);

        self.regs.a = value;
        self.update_result_flags(self.regs.a);
    }
     
    pub(crate) fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);

        self.regs.x = value;
        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        let value = self.memory.read(addr);

        self.regs.y = value;
        self.update_result_flags(self.regs.y);
    }











    pub(crate) fn tax(&mut self) {
        self.regs.x = self.regs.a;
        self.update_result_flags(self.regs.x);
    }

    pub(crate) fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_op_addr(mode);
        self.memory.write(addr, self.regs.a);
    }

}
