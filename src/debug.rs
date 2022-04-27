use crate::{cpu::{AddressingMode, CPU}, bus::Memory};

#[allow(dead_code)]
impl<'a> CPU<'a> {
    pub fn get_absolute_address(&mut self, mode: &AddressingMode, addr: u16) -> (u16, bool) {
        match mode {
            AddressingMode::Immediate => (addr, false),
            AddressingMode::ZeroPage  => (self.read(addr) as u16, false),
            AddressingMode::Absolute => (self.read_16(addr), false),
            AddressingMode::ZeroPageX => {
                let pos = self.read(addr);
                let addr = pos.wrapping_add(self.regs.x) as u16;
                (addr, false)
            },
            AddressingMode::ZeroPageY => {
                let pos = self.read(addr);
                let addr = pos.wrapping_add(self.regs.y) as u16;
                (addr, false)
            },
            AddressingMode::AbsoluteX => {
                let base = self.read_16(addr);
                let addr = base.wrapping_add(self.regs.x as u16);

                let page_crossed = base & 0xFF00 != addr & 0xFF00;
                (addr, page_crossed)
            },
            AddressingMode::AbsoluteY => {
                let base = self.read_16(addr);
                let addr = base.wrapping_add(self.regs.y as u16);

                let page_crossed = base & 0xFF00 != addr & 0xFF00;
                (addr, page_crossed)
            },
            AddressingMode::IndirectX => {
                let base = self.read(addr);
 
                let ptr: u8 = (base as u8).wrapping_add(self.regs.x);
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);
                ((hi as u16) << 8 | (lo as u16), false)
            },
            AddressingMode::IndirectY => {
                let base = self.read(addr);
 
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

    pub fn trace(self: &mut CPU<'a>) -> String {

        let code = self.read(self.regs.pc);
        let ops = self.get_instruction(code);

        let begin = self.regs.pc;
        let mut hex_dump = vec![code];

        let (mem_addr, stored_value) = match ops.addr_mode {
            AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
            _ => {
                let (addr, _) = self.get_absolute_address(&ops.addr_mode, begin + 1);
                (addr, self.read(addr))
            }
        };

        let tmp = match ops.bytes {
            1 => match code {
                0x0a | 0x4a | 0x2a | 0x6a => "A ".to_string(),
                _ => String::from(""),
            },
            2 => {
                let address: u8 = self.read(begin + 1);
                // let value = self.mem_read(address));
                hex_dump.push(address);

                match ops.addr_mode {
                    AddressingMode::Immediate => format!("#${:02x}", address),
                    AddressingMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                    AddressingMode::ZeroPageX => format!(
                        "${:02x},X @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::ZeroPageY => format!(
                        "${:02x},Y @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::IndirectX => format!(
                        "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                        address,
                        (address.wrapping_add(self.regs.x)),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::IndirectY => format!(
                        "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                        address,
                        (mem_addr.wrapping_sub(self.regs.y as u16)),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::NoneAddressing => {
                        // assuming local jumps: BNE, BVS, etc....
                        let address: usize =
                            (begin as usize + 2).wrapping_add((address as i8) as usize);
                        format!("${:04x}", address)
                    }

                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                        ops.addr_mode, code
                    ),
                }
            }
            3 => {
                let address_lo = self.read(begin + 1);
                let address_hi = self.read(begin + 2);
                hex_dump.push(address_lo);
                hex_dump.push(address_hi);

                let address = self.read_16(begin + 1);

                match ops.addr_mode {
                    AddressingMode::NoneAddressing => {
                        if code == 0x6c {
                            //jmp indirect
                            let jmp_addr = if address & 0x00FF == 0x00FF {
                                let lo = self.read(address);
                                let hi = self.read(address & 0xFF00);
                                (hi as u16) << 8 | (lo as u16)
                            } else {
                                self.read_16(address)
                            };

                            // let jmp_addr = self.mem_read_u16(address);
                            format!("(${:04x}) = {:04x}", address, jmp_addr)
                        } else {
                            format!("${:04x}", address)
                        }
                    }
                    AddressingMode::Absolute => format!("${:04x} = {:02x}", mem_addr, stored_value),
                    AddressingMode::AbsoluteX => format!(
                        "${:04x},X @ {:04x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::AbsoluteY => format!(
                        "${:04x},Y @ {:04x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                        ops.addr_mode, code
                    ),
                }
            }
            _ => String::from(""),
        };

        let hex_str = hex_dump
            .iter()
            .map(|z| format!("{:02x}", z))
            .collect::<Vec<String>>()
            .join(" ");
        let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, ops.name, tmp)
            .trim()
            .to_string();

        format!(
            "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
            asm_str, self.regs.a, self.regs.x, self.regs.y, self.regs.p, self.regs.sp,
        )
        .to_ascii_uppercase()
    }
}