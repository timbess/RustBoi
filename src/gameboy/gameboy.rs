use super::cpu::Cpu;
use super::memory::Memory;

pub struct Gameboy {
    cpu: Cpu,
    memory: Memory
}

impl Gameboy {
    pub fn new(rom: Vec<u8>, bootrom: Vec<u8>) -> Self {
        Gameboy {
            cpu: Cpu::new(),
            memory: Memory::new(rom, bootrom)
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.memory);
        }
    }
}
