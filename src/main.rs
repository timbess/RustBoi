mod gameboy;

use std::fs::File;
use std::io::Read;
use gameboy::Gameboy;


fn main() {
    let mut rom_bytes: Vec<u8> = Vec::new();
    let mut bios_bytes: Vec<u8> = Vec::new();
    let mut rom = File::open("roms/tetris.gb").unwrap();
    rom.read_to_end(&mut rom_bytes).unwrap();
    let mut bios = File::open("roms/bios.gb").unwrap();
    bios.read_to_end(&mut bios_bytes).unwrap();

    let mut gameboy = Gameboy::new(rom_bytes, bios_bytes);
    gameboy.run();
}
