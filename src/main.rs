use std::env;
use std::fs::File;
use std::io::prelude::*;

mod cpu;

/// Read a cartridge file and return the entire file as a 'Vec<u8'.
fn read_cartridge(file: String) -> Vec<u8> {
    let mut f = File::open(file).expect("file not found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Could not read file");

    buffer
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // eventually this will need to be changed to an actual
    // command line parser
    if args.len() < 2 {
        println!("Please specify a file");
    } else {
        let binary_data = read_cartridge(args.remove(1));
        let cpu = cpu::Cpu::new(binary_data, 100);
        cpu.execute();
    }
}
