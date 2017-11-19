use super::cpu::Cpu;
use super::memory::Memory;

pub struct Gameboy {
    cpu: Cpu,
    memory: Memory
}

impl Gameboy {
    pub fn new(rom: Vec<u8>, cart_rom: Vec<u8>) -> Self {
        Gameboy {
            cpu: Cpu::new(),
            memory: Memory::new(rom, cart_rom)
        }
    }

    pub fn run(&mut self) {
        self.cpu.step(&mut self.memory);
    }
}