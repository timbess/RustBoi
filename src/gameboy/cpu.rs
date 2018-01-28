use super::memory::Memory;
use super::utils::bit_is_set;

pub struct Cpu {
    af: ComboRegister,
    bc: ComboRegister,
    de: ComboRegister,
    hl: ComboRegister,
    sp: u16,
    pc: u16,

    flags: Flags
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
            flags: Flags::new()
        }
    }

    pub fn step(&mut self, memory: &mut Memory) {
        if self.pc == 0x0100 {
            memory.executed_bios();
        }
        let opcode = self.read_u8_at_pc(memory);
        println!("opcode: {:#x}", opcode);
        match opcode {
            0x31 => { // LD sp, nn
                self.sp = self.read_u16_at_pc(memory);
            }
            0xaf | 0xa8 | 0xa9 | 0xaa | 0xab | 0xac | 0xad | 0xae => { // XOR a
                self.af.hi ^= self.get_register_value(memory, opcode);

                self.flags.zero = self.af.hi == 0x00;
                self.flags.subtract = false;
                self.flags.half_carry = false;
                self.flags.carry = false;
            }
            0x21 => { // LD HL, nn
                let data = self.read_u16_at_pc(memory);
                self.hl.set_combined(data);
            }
            0x32 => { // LDD (hl), a
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
                        self.flags.zero = !bit_is_set(register, bit_to_check);
                        self.flags.half_carry = true;
                        self.flags.subtract = false;
                    }
                    _ => panic!("Unknown special opcode: {:#x}", special_op)
                }
            }
            0x20 | 0x28 | 0x30 | 0x38 => { // JR cc, n
                let offset = self.read_u8_at_pc(memory) as i8;
                match (opcode >> 3) & 0x03 {
                    0x0 => { // NZ
                        if self.flags.zero { return; }
                    }
                    0x1 => { // Z
                        if !self.flags.zero { return; }
                    }
                    0x2 => { // NC
                        if self.flags.carry { return; }
                    }
                    0x3 => { // C
                        if !self.flags.carry { return; }
                    }
                    _ => { panic!("Invalid Jump opcode: {:#x}", opcode); }
                }
                println!("Jumping by offset: '{:#x}'", offset);
                self.pc = (self.pc as i16 + offset as i16) as u16;
            }
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
                        if self.flags.zero { return; }
                    }
                    0x1 => { // Z
                        if !self.flags.zero { return; }
                    }
                    0x2 => { // NC
                        if self.flags.carry { return; }
                    }
                    0x3 => { // C
                        if !self.flags.carry { return; }
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
        let higher = self.pop_stack_u8(memory);
        let lower = self.pop_stack_u8(memory);
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

    fn read_u16_at_pc(&mut self, memory: &mut Memory) -> u16 {
        let current_pc = self.pc;
        self.pc += 2;
        return memory.read_u16(current_pc);
    }
}

struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl Flags {
    fn new() -> Self {
        Flags {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
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
}