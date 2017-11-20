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
            sp: 0,
            pc: 0,
            flags: Flags::new()
        }
    }

    pub fn step(&mut self, memory: &mut Memory) {
        if self.pc == 0x0100 {
            memory.executed_bios();
        }
        let opcode = self.read_u8_at_pc(memory);
        match opcode {
            0x31 => { // LD sp, nn
                self.sp = self.read_u16_at_pc(memory);
            }
            0xaf => { // XOR a
                self.af.hi ^= self.af.hi;
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
                match special_op {
                    0x40 ... 0x7f => { // BIT b, r operations
                        let register = match (special_op & 0x0f) % 0x08 {
                            0x00 => self.bc.hi,
                            0x01 => self.bc.lo,
                            0x02 => self.de.hi,
                            0x03 => self.de.lo,
                            0x04 => self.hl.hi,
                            0x05 => self.hl.lo,
                            0x06 => memory.read_u8(self.hl.get_combined()),
                            0x07 => self.af.hi,
                            _ => panic!("How the fuck did you break modulus?")
                        };
                        let bit_to_check = (special_op - 0x40) / 0x08;

                        self.flags.zero = !bit_is_set(register, bit_to_check);
                        self.flags.half_carry = true;
                        self.flags.subtract = false;
                    }
                    _ => panic!("Unknown special opcode: {:#x}", special_op)
                }
            }
            0x20 => { // JR NZ, n
                if !self.flags.zero {
                    let offset = self.read_u8_at_pc(memory);
                    self.pc += offset as u16;
                }
            }
            0x40 => { // LD B, B
                self.bc.hi = self.bc.hi;
            }
            0x41 => { // LD B, C
                self.bc.hi = self.bc.lo;
            }
            0x42 => { // LD B, D
                self.bc.hi = self.de.hi;
            }
            0x43 => { // LD B, E
                self.bc.hi = self.de.lo;
            }
            0x44 => { // LD B, H
                self.bc.hi = self.hl.hi;
            }
            0x45 => { // LD B, L
                self.bc.hi = self.hl.lo;
            }
            0x46 => { // LD B, (HL)
                self.bc.hi = memory.read_u8(self.hl.get_combined());
            }
            0x47 => { // LD B, A
                self.bc.hi = self.af.hi;
            }
            0x48 => { // LD C, B
                self.bc.lo = self.bc.hi;
            }
            0x49 => { // LD C, C
                self.bc.lo = self.bc.lo;
            }
            0x4a => { // LD C, D
                self.bc.lo = self.de.hi;
            }
            0x4b => { // LD C, E
                self.bc.lo = self.de.lo;
            }
            0x4c => { // LD C, H
                self.bc.lo = self.hl.hi;
            }
            0x4d => { // LD C, L
                self.bc.lo = self.hl.lo;
            }
            0x4e => { // LD C, (HL)
                self.bc.lo = memory.read_u8(self.hl.get_combined());
            }
            0x4f => { // LD C, A
                self.bc.lo = self.af.hi;
            }
            0x50 => { // LD D, B
                self.de.hi = self.bc.hi;
            }
            0x51 => { // LD D, C
                self.de.hi = self.bc.lo;
            }
            0x52 => { // LD D, D
                self.de.hi = self.de.hi;
            }
            0x53 => { // LD D, E
                self.de.hi = self.de.lo;
            }
            0x54 => { // LD D, H
                self.de.hi = self.hl.hi;
            }
            0x55 => { // LD D, L
                self.de.hi = self.hl.lo;
            }
            0x56 => { // LD D, (HL)
                self.de.hi = memory.read_u8(self.hl.get_combined());
            }
            0x57 => { // LD D, A
                self.de.hi = self.af.hi;
            }
            0x58 => { // LD E, B
                self.de.lo = self.bc.hi;
            }
            0x59 => { // LD E, C
                self.de.lo = self.bc.lo;
            }
            0x5a => { // LD E, D
                self.de.lo = self.de.hi;
            }
            0x5b => { // LD E, E
                self.de.lo = self.de.lo;
            }
            0x5c => { // LD E, H
                self.de.lo = self.hl.hi;
            }
            0x5d => { // LD E, L
                self.de.lo = self.hl.lo;
            }
            0x5e => { // LD E, (HL)
                self.de.lo = memory.read_u8(self.hl.get_combined());
            }
            0x5f => { // LD E, A
                self.de.lo = self.af.hi;
            }
            0x60 => { // LD H, B
                self.hl.hi = self.bc.hi;
            }
            0x61 => { // LD H, C
                self.hl.hi = self.bc.lo;
            }
            0x62 => { // LD H, D
                self.hl.hi = self.de.hi;
            }
            0x63 => { // LD H, E
                self.hl.hi = self.de.lo;
            }
            0x64 => { // LD H, H
                self.hl.hi = self.hl.hi;
            }
            0x65 => { // LD H, L
                self.hl.hi = self.hl.lo;
            }
            0x66 => { // LD H, (HL)
                self.hl.hi = memory.read_u8(self.hl.get_combined());
            }
            0x67 => { // LD H, A
                self.hl.hi = self.af.hi;
            }
            0x68 => { // LD L, B
                self.hl.lo = self.bc.hi;
            }
            0x69 => { // LD L, C
                self.hl.lo = self.bc.hi;
            }
            0x6a => { // LD L, D
                self.hl.lo = self.de.hi;
            }
            0x6b => { // LD L, E
                self.hl.lo = self.de.lo;
            }
            0x6c => { // LD L, H
                self.hl.lo = self.hl.hi;
            }
            0x6d => { // LD L, L
                self.hl.lo = self.hl.lo;
            }
            0x6e => { // LD L, (HL)
                self.hl.lo = memory.read_u8(self.hl.get_combined());
            }
            0x6f => { // LD L, A
                self.hl.lo = self.af.hi;
            }
            0x70 => { // LD (HL), B
                memory.write_u8(self.hl.get_combined(), self.bc.hi);
            }
            0x71 => { // LD (HL), C
                memory.write_u8(self.hl.get_combined(), self.bc.lo);
            }
            0x72 => { // LD (HL), D
                memory.write_u8(self.hl.get_combined(), self.de.hi);
            }
            0x73 => { // LD (HL), E
                memory.write_u8(self.hl.get_combined(), self.de.lo);
            }
            0x74 => { // LD (HL), H
                memory.write_u8(self.hl.get_combined(), self.hl.hi);
            }
            0x75 => { // LD (HL), L
                memory.write_u8(self.hl.get_combined(), self.hl.lo);
            }
            0x76 => { // HALT
                panic!("HALT not implemented"); // TODO
            }
            0x77 => { // LD (HL), A
                memory.write_u8(self.hl.get_combined(), self.af.hi);
            }
            0x78 => { // LD A, B
                self.af.hi = self.bc.hi;
            }
            0x79 => { // LD A, C
                self.af.hi = self.bc.lo;
            }
            0x7a => { // LD A, D
                self.af.hi = self.de.hi;
            }
            0x7b => { // LD A, E
                self.af.hi = self.de.lo;
            }
            0x7c => { // LD A, H
                self.af.hi = self.hl.hi;
            }
            0x7d => { // LD A, L
                self.af.hi = self.hl.lo;
            }
            0x7e => { // LD A, (HL)
                self.af.hi = memory.read_u8(self.hl.get_combined());
            }
            0x7f => { // LD A, A
                self.af.hi = self.af.hi;
            }
            _ => panic!("Unknown opcode: {:#x}", opcode)
        }
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