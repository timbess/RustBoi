use std::fs::File;
use std::io::Read;

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open("roms/tetris.gb").unwrap();
    file.read_to_end(&mut bytes).unwrap();
    for byte in &bytes[0..40] {
        println!("{:#x}", byte);
    }
}
