pub struct Memory {
    main_ram: Box<[u8]>,
    video_ram: Box<[u8]>,
    bios: Box<[u8]>,
    rom: Box<[u8]>,
    executed_bios: bool
}

const RAM_SIZE: usize = 8 * 1024;

impl Memory {
    pub fn new(rom: Vec<u8>, cart_rom: Vec<u8>) -> Self {
        Memory {
            main_ram: vec![0; RAM_SIZE].into_boxed_slice(),
            video_ram: vec![0; RAM_SIZE].into_boxed_slice(),
            bios: cart_rom.into_boxed_slice(),
            rom: rom.into_boxed_slice(),
            executed_bios: false
        }
    }

    pub fn executed_bios(&mut self) {
        self.executed_bios = true;
    }

    fn get_memory_space_with_addr(&self, addr: u16) -> (&Box<[u8]>, u16) {
        match addr & 0xF000 {
            0x0000 => {
                if !self.executed_bios && addr < 0x0100 {
                    return ((&self.bios), addr);
                }

                return ((&self.rom), addr);
            }
            _ => {
                panic!("Unknown memory region: {:#x}", addr);
            }
        }
    }

    fn get_memory_space_with_addr_mut(&mut self, addr: u16) -> (&mut Box<[u8]>, u16) {
        match addr & 0xF000 {
            0x0000 => {
                if !self.executed_bios && addr < 0x0100 {
                    return ((&mut self.bios), addr);
                }

                return ((&mut self.rom), addr);
            }
            _ => {
                panic!("Unknown memory region: {:#x}", addr);
            }
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        let (memory_space, addr) = self.get_memory_space_with_addr(addr);
        return memory_space[addr as usize];
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        let (memory_space, addr) = self.get_memory_space_with_addr_mut(addr);
        return memory_space[addr as usize] = value;
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        return ((self.read_u8(addr+1)   as u16) << 8) |
               ((self.read_u8(addr) as u16));
    }
}