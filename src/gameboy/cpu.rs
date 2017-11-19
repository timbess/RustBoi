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

    pub fn step(&self, memory: &mut Memory) {
        if self.pc == 0x0100 {
            memory.executed_bios();
        }
        let opcode = memory.read_byte(self.pc);
        panic!("{:#x}", opcode);
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
    fn combined(&self) -> u16 {
        return ((self.hi as u16) << 8) |
               ((self.lo as u16));
    }
}