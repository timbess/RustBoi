use super::memory::Memory;
use super::utils::bit_is_set;

pub struct Cpu {
    af: ComboRegister,
    bc: ComboRegister,
    de: ComboRegister,
    hl: ComboRegister,
    sp: u16,
    pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            af: ComboRegister::new(),
            bc: ComboRegister::new(),
            de: ComboRegister::new(),
            hl: ComboRegister::new(),
            sp: 0xfffe,
            pc: 0,
        }
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let opcode_addr = self.pc;
        let opcode = self.read_u8_at_pc(memory);
        println!("{:#x} opcode: {:#x}", opcode_addr, opcode);
        match opcode {
            0xaf | 0xa8 | 0xa9 | 0xaa | 0xab | 0xac | 0xad | 0xae => { // XOR a
                self.af.hi ^= self.get_register_value(memory, opcode);
                let new_zero = self.af.hi == 0x00;
                self.af.set_flag_lo(Flags::Zero(new_zero));
                self.af.set_flag_lo(Flags::Subtract(false));
                self.af.set_flag_lo(Flags::HalfCarry(false));
                self.af.set_flag_lo(Flags::Carry(false));
            }
            0x32 => { // LDD (HL-), A
                memory.write_u8(self.hl.get_combined(), self.af.hi);
                let new_hl = self.hl.get_combined()-1;
                self.hl.set_combined(new_hl);
            }
            0xcb => { // Special multibyte instructions
                let special_op = self.read_u8_at_pc(memory);
                println!("Special opcode: {:#x}", special_op);
                match special_op {
                    0x40 ... 0x7f => { // BIT b, r operations
                        let bit_to_check = (special_op - 0x40) / 0x08;
                        let register = self.get_register_value(memory, special_op);
                        self.af.set_flag_lo(Flags::Zero(!bit_is_set(register, bit_to_check)));
                        self.af.set_flag_lo(Flags::HalfCarry(true));
                        self.af.set_flag_lo(Flags::Subtract(false));
                    }
                    0x11 => { // RL C
                        let carry_bit = (self.bc.lo & 0x80) == 0x80;
                        self.bc.lo <<= 1;
                        self.bc.hi |= self.af.check_flag_low(Flags::Carry(true)) as u8;
                        self.af.set_flag_lo(Flags::Carry(carry_bit));
                    }
                    _ => panic!("Unknown special opcode: {:#x}", special_op)
                }
            }
            0x17 => { // RL A
                let carry_bit = (self.af.hi & 0x80) == 0x80;
                self.af.hi <<= 1;
                self.af.hi |= self.af.check_flag_low(Flags::Carry(true)) as u8;
                self.af.set_flag_lo(Flags::Carry(carry_bit));
            }
            0x20 | 0x28 | 0x30 | 0x38 => { // JR cc, n
                let offset = self.read_i8_at_pc(memory);
                match (opcode >> 3) & 0x03 {
                    0x0 => { // NZ
                        if self.af.check_flag_low(Flags::Zero(true)) { return; }
                    }
                    0x1 => { // Z
                        if self.af.check_flag_low(Flags::Zero(false)) { return; }
                    }
                    0x2 => { // NC
                        if self.af.check_flag_low(Flags::Carry(true)) { return; }
                    }
                    0x3 => { // C
                        if self.af.check_flag_low(Flags::Carry(false)) { return; }
                    }
                    _ => { panic!("Invalid Jump opcode: {:#x}", opcode); }
                }
                println!("Jumping by offset: '{:#x}'", offset);
                self.pc = (self.pc as i16 + offset as i16) as u16;
            }
            0x18 => { // JR n
                let offset = self.read_i8_at_pc(memory);
                println!("Jumping by offset: '{:#x}'", offset);
                self.pc = (self.pc as i16 + offset as i16) as u16;
            }
            0x01 => { // LD BC, nn
                let value = self.read_u16_at_pc(memory);
                self.bc.set_combined(value);
            }
            0x11 => { // LD DE, nn
                let value = self.read_u16_at_pc(memory);
                self.de.set_combined(value);
            }
            0x21 => { // LD HL, nn
                let value = self.read_u16_at_pc(memory);
                self.hl.set_combined(value);
            }
            0x31 => { // LD SP, nn
                self.sp = self.read_u16_at_pc(memory);
            }
            0xe0 => { // LD (n), A
                let addr = 0xff00 + (self.read_u8_at_pc(memory) as u16);
                memory.write_u8(addr, self.af.hi);
            }
            0xea => { // LD (nn), A
                let addr = self.read_u16_at_pc(memory);
                memory.write_u8(addr, self.af.hi);
            }
            0x1a => { self.af.hi = memory.read_u8(self.de.get_combined()); } // LD A, (DE)
            0x06 => { self.bc.hi = self.read_u8_at_pc(memory); } // LD B, n
            0xe2 => { memory.write_u8(0xff00 + (self.bc.lo as u16), self.af.hi); } // LD (C), A
            0x22 => { // LDI (HL), A
                let address = self.hl.get_combined();
                memory.write_u8(address, self.af.hi);
                self.hl.set_combined(address + 1);
             }
            0x0e => { self.bc.lo = self.read_u8_at_pc(memory) } // LD C, n
            0x2e => { self.hl.lo = self.read_u8_at_pc(memory) } // LD L, n
            0x3e => { self.af.hi = self.read_u8_at_pc(memory) } // LD A, n
            0x40 => { self.bc.hi = self.bc.hi; } // LD B, B
            0x41 => { self.bc.hi = self.bc.lo; } // LD B, C
            0x42 => { self.bc.hi = self.de.hi; } // LD B, D
            0x43 => { self.bc.hi = self.de.lo; } // LD B, E
            0x44 => { self.bc.hi = self.hl.hi; } // LD B, H
            0x45 => { self.bc.hi = self.hl.lo; } // LD B, L
            0x46 => { self.bc.hi = memory.read_u8(self.hl.get_combined()); } // LD B, (HL)
            0x47 => { self.bc.hi = self.af.hi; } // LD B, A
            0x48 => { self.bc.lo = self.bc.hi; } // LD C, B
            0x49 => { self.bc.lo = self.bc.lo; } // LD C, C
            0x4a => { self.bc.lo = self.de.hi; } // LD C, D
            0x4b => { self.bc.lo = self.de.lo; } // LD C, E
            0x4c => { self.bc.lo = self.hl.hi; } // LD C, H
            0x4d => { self.bc.lo = self.hl.lo; } // LD C, L
            0x4e => { self.bc.lo = memory.read_u8(self.hl.get_combined()); } // LD C, (HL)
            0x4f => { self.bc.lo = self.af.hi; } // LD C, A
            0x50 => { self.de.hi = self.bc.hi; } // LD D, B
            0x51 => { self.de.hi = self.bc.lo; } // LD D, C
            0x52 => { self.de.hi = self.de.hi; } // LD D, D
            0x53 => { self.de.hi = self.de.lo; } // LD D, E
            0x54 => { self.de.hi = self.hl.hi; } // LD D, H
            0x55 => { self.de.hi = self.hl.lo; } // LD D, L
            0x56 => { self.de.hi = memory.read_u8(self.hl.get_combined()); } // LD D, (HL)
            0x57 => { self.de.hi = self.af.hi; } // LD D, A
            0x58 => { self.de.lo = self.bc.hi; } // LD E, B
            0x59 => { self.de.lo = self.bc.lo; } // LD E, C
            0x5a => { self.de.lo = self.de.hi; } // LD E, D
            0x5b => { self.de.lo = self.de.lo; } // LD E, E
            0x5c => { self.de.lo = self.hl.hi; } // LD E, H
            0x5d => { self.de.lo = self.hl.lo; } // LD E, L
            0x5e => { self.de.lo = memory.read_u8(self.hl.get_combined()); } // LD E, (HL)
            0x5f => { self.de.lo = self.af.hi; } // LD E, A
            0x60 => { self.hl.hi = self.bc.hi; } // LD H, B
            0x61 => { self.hl.hi = self.bc.lo; } // LD H, C
            0x62 => { self.hl.hi = self.de.hi; } // LD H, D
            0x63 => { self.hl.hi = self.de.lo; } // LD H, E
            0x64 => { self.hl.hi = self.hl.hi; } // LD H, H
            0x65 => { self.hl.hi = self.hl.lo; } // LD H, L
            0x66 => { self.hl.hi = memory.read_u8(self.hl.get_combined()); } // LD H, (HL)
            0x67 => { self.hl.hi = self.af.hi; } // LD H, A
            0x68 => { self.hl.lo = self.bc.hi; } // LD L, B
            0x69 => { self.hl.lo = self.bc.hi; } // LD L, C
            0x6a => { self.hl.lo = self.de.hi; } // LD L, D
            0x6b => { self.hl.lo = self.de.lo; } // LD L, E
            0x6c => { self.hl.lo = self.hl.hi; } // LD L, H
            0x6d => { self.hl.lo = self.hl.lo; } // LD L, L
            0x6e => { self.hl.lo = memory.read_u8(self.hl.get_combined()); } // LD L, (HL)
            0x6f => { self.hl.lo = self.af.hi; } // LD L, A
            0x70 => { memory.write_u8(self.hl.get_combined(), self.bc.hi); } // LD (HL), B
            0x71 => { memory.write_u8(self.hl.get_combined(), self.bc.lo); } // LD (HL), C
            0x72 => { memory.write_u8(self.hl.get_combined(), self.de.hi); } // LD (HL), D
            0x73 => { memory.write_u8(self.hl.get_combined(), self.de.lo); } // LD (HL), E
            0x74 => { memory.write_u8(self.hl.get_combined(), self.hl.hi); } // LD (HL), H
            0x75 => { memory.write_u8(self.hl.get_combined(), self.hl.lo); } // LD (HL), L
            0x76 => { panic!("HALT not implemented"); } // TODO HALT
            0x77 => { memory.write_u8(self.hl.get_combined(), self.af.hi); } // LD (HL), A
            0x78 => { self.af.hi = self.bc.hi; } // LD A, B
            0x79 => { self.af.hi = self.bc.lo; } // LD A, C
            0x7a => { self.af.hi = self.de.hi; } // LD A, D
            0x7b => { self.af.hi = self.de.lo; } // LD A, E
            0x7c => { self.af.hi = self.hl.hi; } // LD A, H
            0x7d => { self.af.hi = self.hl.lo; } // LD A, L
            0x7e => { self.af.hi = memory.read_u8(self.hl.get_combined()); } // LD A, (HL)
            0x7f => { self.af.hi = self.af.hi; } // LD A, A
            0xc4 | 0xcc | 0xd4 | 0xdc => { // CALL cc, nn
                let jump_to_addr = self.read_u16_at_pc(memory);
                match (opcode >> 3) & 0x03 {
                    0x0 => { // NZ
                        if self.af.check_flag_low(Flags::Zero(true)) { return; }
                    }
                    0x1 => { // Z
                        if self.af.check_flag_low(Flags::Zero(false)) { return; }
                    }
                    0x2 => { // NC
                        if self.af.check_flag_low(Flags::Carry(true)) { return; }
                    }
                    0x3 => { // C
                        if self.af.check_flag_low(Flags::Carry(false)) { return; }
                    }
                    _ => { panic!("Invalid Call opcode: {:#x}", opcode); }
                }
                println!("jumping to: {:#x}", jump_to_addr);
                self.call(memory, jump_to_addr);
            }
            0xcd => { // CALL nn
                let jump_to_addr = self.read_u16_at_pc(memory);
                self.call(memory, jump_to_addr);
            }
            0x23 => { // INC HL
                let half_carry_bit = self.hl.get_combined() & 0x0800 == 0x0800;
                let new_hl = self.hl.get_combined() + 1;
                self.hl.set_combined(new_hl);
                self.af.set_flag_lo(Flags::Zero(new_hl == 0));
                self.af.set_flag_lo(Flags::Subtract(false));
                self.af.set_flag_lo(Flags::HalfCarry(half_carry_bit && ((new_hl & 0x0800) == 0)));
                self.af.set_flag_lo(Flags::Carry(new_hl == 0));
            }
            0x13 => { // INC DE
                let half_carry_bit = self.de.get_combined() & 0x0800 == 0x0800;
                let new_de = self.de.get_combined() + 1;
                self.de.set_combined(new_de);
                self.af.set_flag_lo(Flags::Zero(new_de == 0));
                self.af.set_flag_lo(Flags::Subtract(false));
                self.af.set_flag_lo(Flags::HalfCarry(half_carry_bit && ((new_de & 0x0800) == 0)));
                self.af.set_flag_lo(Flags::Carry(new_de == 0));
            }
            0x3c | 0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c => { // INC n
                let (result, half_carry_bit) = {
                    let reg = match opcode >> 3 {
                        0 => { &mut self.bc.hi }
                        1 => { &mut self.bc.lo }
                        2 => { &mut self.de.hi }
                        3 => { &mut self.de.lo }
                        4 => { &mut self.hl.hi }
                        5 => { &mut self.hl.lo }
                        7 => { &mut self.af.hi }
                        _ => { unreachable!() }
                    };
                    let half_carry_bit = (*reg & 0x08) == 0x08;
                    *reg = (*reg).wrapping_add(1);
                    (*reg, half_carry_bit)
                };
                self.af.set_flag_lo(Flags::Zero(result == 0));
                self.af.set_flag_lo(Flags::Subtract(false));
                self.af.set_flag_lo(Flags::HalfCarry(half_carry_bit && ((result & 0x08) == 0)));
                self.af.set_flag_lo(Flags::Carry(result == 0));
            }
            0x3d | 0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d => { // DEC n
                let (result, half_carry_bit) = {
                    let reg = match opcode >> 3 {
                        0 => { &mut self.bc.hi }
                        1 => { &mut self.bc.lo }
                        2 => { &mut self.de.hi }
                        3 => { &mut self.de.lo }
                        4 => { &mut self.hl.hi }
                        5 => { &mut self.hl.lo }
                        7 => { &mut self.af.hi }
                        _ => { unreachable!() }
                    };
                    let half_carry_bit = (*reg & 0x08) == 0;
                    *reg = (*reg).wrapping_sub(1);
                    (*reg, half_carry_bit)
                };
                self.af.set_flag_lo(Flags::Zero(result == 0));
                self.af.set_flag_lo(Flags::Subtract(true));
                self.af.set_flag_lo(Flags::HalfCarry(half_carry_bit && ((result & 0x08) == 0x08)));
                self.af.set_flag_lo(Flags::Carry(result == 0xff));
            }
            0xc5 => { // PUSH BC
                let value = self.bc.get_combined();
                self.push_stack_u16(memory, value);
            }
            0xc1 => { // POP BC
                let value = self.pop_stack_u16(memory);
                self.bc.set_combined(value);
            }
            0xc9 => { // RET
                let addr = self.pop_stack_u16(memory);
                self.pc = addr;
            }
            0xbf | 0xb8 | 0xb9 | 0xba | 0xbb | 0xbc | 0xbd | 0xbe => { // CP n
                let compared_register = self.get_register_value(memory, opcode);
                let half_carry_bit = (self.af.hi & 0x08) == 0;
                let af_hi = self.af.hi;
                let res = self.af.hi.wrapping_sub(compared_register);
                self.af.set_flag_lo(Flags::Zero(res == 0));
                self.af.set_flag_lo(Flags::Subtract(true));
                self.af.set_flag_lo(Flags::HalfCarry(half_carry_bit && ((res & 0x08) == 0x08)));
                self.af.set_flag_lo(Flags::Carry(res >= af_hi));
            }
            0xfe => { // CP #
                let compared_value = self.read_u8_at_pc(memory);
                let half_carry_bit = (self.af.hi & 0x08) == 0;
                let af_hi = self.af.hi;
                let res = self.af.hi.wrapping_sub(compared_value);
                self.af.set_flag_lo(Flags::Zero(res == 0));
                self.af.set_flag_lo(Flags::Subtract(true));
                self.af.set_flag_lo(Flags::HalfCarry(half_carry_bit && ((res & 0x08) == 0x08)));
                self.af.set_flag_lo(Flags::Carry(res >= af_hi));
            }
            _ => panic!("Unknown opcode: {:#x}", opcode)
        }
    }

    fn get_register_value(&mut self, memory: &mut Memory, opcode: u8) -> u8 {
        match (opcode & 0x0f) % 0x08 {
            0x00 => self.bc.hi,
            0x01 => self.bc.lo,
            0x02 => self.de.hi,
            0x03 => self.de.lo,
            0x04 => self.hl.hi,
            0x05 => self.hl.lo,
            0x06 => memory.read_u8(self.hl.get_combined()),
            0x07 => self.af.hi,
            _ => panic!("How the fuck did you break modulus?")
        }
    }

    fn call(&mut self, memory: &mut Memory, jump_to_addr: u16) {
        let current_pc = self.pc;
        self.push_stack_u16(memory, current_pc);
        self.pc = jump_to_addr;
    }

    fn push_stack_u16(&mut self, memory: &mut Memory, value: u16) {
        self.push_stack_u8(memory, (value >> 8) as u8);
        self.push_stack_u8(memory, (value & 0x00FF) as u8);
    }

    fn pop_stack_u16(&mut self, memory: &mut Memory) -> u16 {
        let lower = self.pop_stack_u8(memory);
        let higher = self.pop_stack_u8(memory);
        ((higher as u16) << 8) | ((lower as u16) & 0x00ff)
    }

    fn push_stack_u8(&mut self, memory: &mut Memory, value: u8) {
        memory.write_u8(self.sp, value);
        self.sp -= 1;
    }

    fn pop_stack_u8(&mut self, memory: &mut Memory) -> u8 {
        self.sp += 1;
        memory.read_u8(self.sp)
    }

    fn read_u8_at_pc(&mut self, memory: &mut Memory) -> u8 {
        let current_pc = self.pc;
        self.pc += 1;
        return memory.read_u8(current_pc);
    }

    fn read_i8_at_pc(&mut self, memory: &mut Memory) -> i8 {
        let current_pc = self.pc;
        self.pc += 1;
        return memory.read_u8(current_pc) as i8;
    }

    fn read_u16_at_pc(&mut self, memory: &mut Memory) -> u16 {
        let current_pc = self.pc;
        self.pc += 2;
        return memory.read_u16(current_pc);
    }
}

enum Flags {
    Zero(bool),
    Subtract(bool),
    HalfCarry(bool),
    Carry(bool)
}

struct ComboRegister {
    hi: u8,
    lo: u8,
}

impl ComboRegister {
    fn new() -> Self {
        ComboRegister {
            hi: 0,
            lo: 0
        }
    }

    fn get_combined(&self) -> u16 {
        return ((self.hi as u16) << 8) |
               ((self.lo as u16) & 0x00ff);
    }

    fn set_combined(&mut self, combined: u16) {
        self.hi = (combined >> 8) as u8;
        self.lo = (combined & 0x00ff) as u8;
    }

    fn set_flag_lo(&mut self, flag: Flags) {
        match flag {
            Flags::Zero(value) => { self.lo = (!(1<<7) & self.lo) | ((value as u8) << 7) }
            Flags::Subtract(value) => { self.lo = (!(1<<6) & self.lo) | ((value as u8) << 6) }
            Flags::HalfCarry(value) => { self.lo = (!(1<<5) & self.lo) | ((value as u8) << 5) }
            Flags::Carry(value) => { self.lo = (!(1<<4) & self.lo) | ((value as u8) << 4) }
        }
    }

    fn check_flag_low(&self, flag: Flags) -> bool {
        match flag {
            Flags::Zero(value) => {
                let check_bit = (value as u8) << 7;
                (self.lo & (1<<7)) == check_bit
            }
            Flags::Subtract(value) => {
                let check_bit = (value as u8) << 6;
                (self.lo & (1<<6)) == check_bit
            }
            Flags::HalfCarry(value) => {
                let check_bit = (value as u8) << 5;
                (self.lo & (1<<5)) == check_bit
            }
            Flags::Carry(value) => {
                let check_bit = (value as u8) << 4;
                (self.lo & (1<<4)) == check_bit
            }
        }
    }
}