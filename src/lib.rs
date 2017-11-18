mod cpu;
mod memory;

use cpu::Cpu;

const RAM_SIZE: u32 = 8 * 1024

pub struct Gameboy {
    cpu: Cpu,
    rom: Box<[u8]>,
    main_ram: Box<[u8]>,
    video_ram: Box<[u8]>
}

impl Gameboy {
    pub fn new(rom: Vec<u8>, cart_rom: Vec<u8>) {
        Gameboy {
            cpu: Cpu::new(),
            main_ram: vec![0; RAM_SIZE].into_boxed_slice(),
            video_ram: vec![0; RAM_SIZE].into_boxed_slice(),
            rom: rom.into_boxed_slice()
        }
    }
}