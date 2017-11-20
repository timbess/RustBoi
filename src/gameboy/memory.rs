pub struct Memory {
    main_ram: Box<[u8]>,
    video_ram: Box<[u8]>,
    bios: Box<[u8]>,
    zero_page: Box<[u8]>,
    rom: Box<[u8]>,
    executed_bios: bool
}

const RAM_SIZE: usize = 8 * 1024;
const ZERO_PAGE_SIZE: usize = 8 * 128;

impl Memory {
    pub fn new(rom: Vec<u8>, cart_rom: Vec<u8>) -> Self {
        Memory {
            main_ram: vec![0; RAM_SIZE].into_boxed_slice(),
            video_ram: vec![0; RAM_SIZE].into_boxed_slice(),
            bios: cart_rom.into_boxed_slice(),
            zero_page: vec![0; ZERO_PAGE_SIZE].into_boxed_slice(),
            rom: rom.into_boxed_slice(),
            executed_bios: false
        }
    }

    pub fn executed_bios(&mut self) {
        self.executed_bios = true;
    }

    fn get_memory_space_with_addr(&mut self, addr: u16) -> (&mut Box<[u8]>, u16) {
        match addr & 0xF000 {
            0x0000 ... 0x7fff => {
                if !self.executed_bios && addr < 0x0100 {
                    return ((&mut self.bios), addr);
                }

                return ((&mut self.rom), addr);
            }
            0x8000 ... 0x9fff => {
                return ((&mut self.video_ram), addr - 0x8000);
            }
            0xFF80 ... 0xfffe => {
                return ((&mut self.zero_page), addr - 0x0FF80);
            }
            _ => {
                panic!("Unknown memory region: {:#x}", addr);
            }
        }
    }

    pub fn read_u8(&mut self, addr: u16) -> u8 {
        let (memory_space, addr) = self.get_memory_space_with_addr(addr);
        return memory_space[addr as usize];
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        let (memory_space, addr) = self.get_memory_space_with_addr(addr);
        return memory_space[addr as usize] = value;
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
        return ((self.read_u8(addr+1)   as u16) << 8) |
               ((self.read_u8(addr) as u16));
    }

    pub fn dump(&self) {
        for bytes in Vec::from(self.main_ram.as_ref()).chunks(15) {
            for byte in bytes {
                print!("{:#x} ", byte);
            }
            println!();
        }
        for bytes in Vec::from(self.video_ram.as_ref()).chunks(15) {
            for byte in bytes {
                print!("{:#x} ", byte);
            }
            println!();
        }
    }
}