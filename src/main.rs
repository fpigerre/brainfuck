use std::io;

use std::env;
use std::error::Error;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};

use std::str;

const INCREMENT_POINTER: u8 = 0x3E;
const DECREMENT_POINTER: u8 = 0x3C;
const INCREMENT_VALUE : u8 = 0x2B;
const DECREMENT_VALUE : u8 = 0x2D;
const OUTPUT_VALUE : u8 = 0x2E;
const ACCEPT_INPUT : u8 = 0x2C;
const JUMP_FORWARD : u8 = 0x5B;
const JUMP_BACKWARDS : u8 = 0x5D;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Filename required");
    }

    let filename = &args[1];
    let path = Path::new(filename);

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    interpret_commands(&mut buf_reader);

    Ok(())
}

fn interpret_commands(buf_reader: &mut BufReader<File>) {
    // Initialise 30, 000 8-bit cells
    let mut cells: Vec<i8> = Vec::with_capacity(30000);
    let mut data_pointer: i8;

    for byte in buf_reader.bytes() {
        // For now we assume each character is in ASCII and is one byte long
        match byte {
            INCREMENT_POINTER => {data_pointer += 1},
            DECREMENT_POINTER => {data_pointer -= 1},
            INCREMENT_VALUE=> {cells[data_pointer] += 1},
            DECREMENT_VALUE=> {cells[data_pointer] -= 1},
            OUTPUT_VALUE=> {},
            ACCEPT_INPUT => {},
            JUMP_FORWARD=> {},
            JUMP_BACKWARDS=> {}
        }
    }
}