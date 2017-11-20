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
        let opcode = memory.read_u8(self.pc);
        self.pc += 1;
        match opcode {
            0x31 => { // LD sp, nn
                self.sp = memory.read_u16(self.pc);
                self.pc += 2;
            }
            0xaf => { // XOR a
                self.af.hi ^= self.af.hi;
            }
            0x21 => { // LD HL, nn
                self.hl.set_combined(memory.read_u16(self.pc));
                self.pc += 2;
            }
            0x32 => { // LDD (hl), a
                memory.write_u8(self.hl.get_combined(), self.af.hi);
                let new_hl = self.hl.get_combined()-1;
                self.hl.set_combined(new_hl);
            }
            0xcb => { // Special multibyte instructions
                let special_op = memory.read_u8(self.pc);
                self.pc += 1;
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
                    let offset = memory.read_u8(self.pc);
                    self.pc += offset as u16;
                }
            }
            _ => panic!("Unknown opcode: {:#x}", opcode)
        }
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