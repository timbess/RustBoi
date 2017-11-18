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
    pub fn new() {
        Cpu {
            af: ComboRegister::new(),
            bc: ComboRegister::new(),
            de: ComboRegister::new(),
            hl: ComboRegister::new(),
            sp: 0,
            pc: 0x100,
            flags: Flags::new()
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
    fn new() {
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
    fn new() {
        ComboRegister {
            hi: 0,
            lo: 0
        }
    }
    fn combined(&self) {
        return ((self.hi as u16) << 8) |
               ((self.lo as u16));
    }
}