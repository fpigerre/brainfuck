extern crate num;

use std::io;

use std::env;
use std::error::Error;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use std::str;

enum Instruction {
    IncrementPointer = 0x3E,
    DecrementPointer = 0x3C,
    IncrementValue = 0x2B,
    DecrementValue = 0x2D,
    OutputValue = 0x2E,
    AcceptInput = 0x2C,
    JumpForward = 0x5B,
    JumpBackward = 0x5D
}

impl Instruction {
    fn from_u8(value: u8) -> Option<Instruction> {
        let instruction: Option<Instruction> = num::FromPrimitive::from_u8(value);
        instruction
    }
}

// Initialise 30, 000 8-bit cells at runtime
static mut CELLS: Vec<u8> = Vec::with_capacity(30000);
static mut DATA_POINTER: usize = 0;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Filename required");
    }

    let filename = &args[1];
    let path = Path::new(filename);

    let file = File::open(path)?; // ? Means push errors up the stack
    let mut buf_reader = BufReader::new(file);

    interpret_program(&mut buf_reader);

    Ok(())
}

fn interpret_program(buf_reader: &mut BufReader<File>) {
    for byte in buf_reader.bytes() {
        // For now we assume each character is in ASCII and is one byte long
        let instruction : Option<Instruction> = Instruction::from_u8(byte.unwrap());
        match instruction {
            Some(valid_instruction) => interpret_instruction(valid_instruction),
            None => panic!("Unrecognised instruction")
        }
    }
}

fn interpret_instruction(instruction: Instruction) {
    use Instruction::*;
    match instruction {
        IncrementPointer => { data_pointer += 1 },
        DecrementPointer => { data_pointer -= 1 },
        IncrementValue => { cells[data_pointer] += 1 },
        DecrementValue => { cells[data_pointer] -= 1 },
        OutputValue => io::stdout().write(&cells[data_pointer]),
        AcceptInput => io::stdin().read(&mut cells[data_pointer]),
        JumpForward => {},
        JumpBackward => {}
    }
}