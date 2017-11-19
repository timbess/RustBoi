use super::memory::Memory;

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
            }
            _ => panic!("Unknown opcode: {:#x}", opcode)
        }
    }
}

struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Flags {
    fn new() -> Self {
        Flags {
            z: false,
            n: false,
            h: false,
            c: false,
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