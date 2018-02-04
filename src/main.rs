mod gameboy;

use std::fs::File;
use std::io::Read;
use gameboy::Gameboy;


fn main() {
    let mut rom_bytes: Vec<u8> = Vec::new();
    let mut bootrom_bytes: Vec<u8> = Vec::new();
    let mut rom = File::open("roms/tetris.gb").unwrap();
    rom.read_to_end(&mut rom_bytes).unwrap();
    let mut bootrom = File::open("roms/bootrom.gb").unwrap();
    bootrom.read_to_end(&mut bootrom_bytes).unwrap();

    let mut gameboy = Gameboy::new(rom_bytes, bootrom_bytes);
    gameboy.run();
}
