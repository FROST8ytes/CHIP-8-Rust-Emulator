use std::fs::File;
use std::io::Read;
use std::process;

mod chip8;
mod memory;

fn main() {
    let mut file = File::open("ROM/INVADERS").unwrap_or_else( |e| {
        eprintln!("File read error: {e}");
        process::exit(1);
    });

    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap_or_else(|e| {
        eprintln!("File read error: {e}");
        process::exit(1);
    });

    println!("Data: {:?}", data);
    println!("Length of data: {}", data.len());

    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(&data);
}
