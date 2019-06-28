use std::io;

use std::env;
use std::error::Error;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use std::convert::TryFrom;

pub mod instruction;

struct Program {
    cells: Vec<u8>,
    data_pointer: usize,
    program_iterator: BufReader<File>
}

// TODO: Yes, I know I'm forcing myself to use OOP for Rust
// Yes, I will eventually redesign this and make it less OOP and more functional/data-oriented
impl Program {
    fn parse_instruction(&mut self, instruction: instruction::Instruction) {
        use instruction::Instruction::*;
        match instruction {
            IncrementPointer => self.increment_pointer(),
            DecrementPointer => self.decrement_pointer(),
            IncrementValue => self.increment_value(),
            DecrementValue => self.decrement_value(),
            OutputValue => self.output_value(),
            AcceptInput => self.accept_input(),
            JumpForward => self.jump_forward(),
            JumpBackward => self.jump_backward()
        }
    }

    fn parse_sequence(self, sequence: Vec<instruction::Instruction>) {

    }

    fn increment_pointer(&mut self) {
        self.data_pointer += 1;
    }

    fn decrement_pointer(&mut self) {
        self.data_pointer -= 1;
    }

    fn increment_value(&mut self) {
        self.cells[self.data_pointer] += 1;
    }

    fn decrement_value(&mut self) {
        self.cells[self.data_pointer] -= 1;
    }

    fn output_value(&mut self) {
        io::stdout().write(&[self.cells[self.data_pointer]]);
    }

    fn accept_input(&mut self) {
        io::stdin().read(&mut [self.cells[self.data_pointer]]);
    }

    fn jump_forward(&mut self) {
        // if the byte at the data pointer is zero,
        // then instead of moving the instruction pointer forward to the next command,
        // jump it forward to the command after the matching ] command.

    }

    fn jump_backward(&mut self) {
        // if the byte at the data pointer is nonzero,
        // then instead of moving the instruction pointer forward to the next command,
        // jump it back to the command after the matching [ command.
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Filename required");
    }

    let filename = &args[1];
    let path = Path::new(filename);

    let file = File::open(path)?; // ? Means push errors up the stack

    // Initialise 30, 000 8-bit cells at runtime
    let cells: Vec<u8> = Vec::with_capacity(30000);
    let data_pointer: usize = 0;
    let program_iterator: BufReader<File> = BufReader::new(file);
    let mut program = Program{  cells, data_pointer, program_iterator };

    interpret_program(&mut program);

    Ok(())
}

fn interpret_program(program: &mut Program) {
    use instruction::Instruction;

    for byte in program.program_iterator.bytes() {
        // For now we assume each character is in ASCII and is one byte long
        let instruction : Instruction = match Instruction::try_from(byte.unwrap()) {
            Ok(valid_instruction) => valid_instruction,
            Err(e) => { println!("{}", e); continue; }
        };

        program.parse_instruction(instruction);
    }
}
