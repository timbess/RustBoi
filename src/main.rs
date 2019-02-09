#[macro_use]
extern crate log;

mod gameboy;

use std::fs::File;
use std::io::Read;
use gameboy::Gameboy;


fn main() {
    env_logger::init();

    let mut rom_bytes: Vec<u8> = Vec::new();
    let mut bootrom_bytes: Vec<u8> = Vec::new();
    let mut rom = File::open("roms/tetris.gb").expect("Could not find Tetris rom");
    rom.read_to_end(&mut rom_bytes).unwrap();
    let mut bootrom = File::open("roms/bootrom.gb").unwrap();
    bootrom.read_to_end(&mut bootrom_bytes).expect("Could not find bootrom");

    let mut gameboy = Gameboy::new(rom_bytes, bootrom_bytes);
    gameboy.run();
}
