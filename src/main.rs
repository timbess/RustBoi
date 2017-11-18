use std::fs::File;
use std::io::Read;

fn main() {
    let mut rom_bytes: Vec<u8> = Vec::new();
    let mut rom = File::open("roms/tetris.gb").unwrap();
    rom.read_to_end(&mut rom_bytes).unwrap();
}
