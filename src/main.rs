use std::io;
use std::io::Bytes;

use std::env;
use std::error::Error;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use std::convert::TryFrom;

pub mod instruction;
use instruction::Instruction;

struct Program {
    cells: Vec<u8>,
    data_pointer: usize,
    byte_iterator: Bytes<BufReader<File>>
}

// TODO: Yes, I know I'm forcing myself to use OOP for Rust
// Yes, I will eventually redesign this and make it less OOP and more functional/data-oriented
impl Program {
    fn interpret_instruction(&mut self, instruction: instruction::Instruction) {
        use Instruction::*;
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

    fn interpret_sequence(&mut self, sequence: Vec<Instruction>) {
        use Instruction::*;

        for instruction in sequence {
            match instruction {
                JumpForward => {},
                JumpBackward => {},
                _ => self.interpret_instruction(instruction)
            }
        }
    }

    /// Recursively builds a sequence of matching bracket pairs
    /// A sequence is a set of instructions between a matching bracket pair
    fn hold_sequence(&mut self) -> Vec<Instruction> {
        let mut sequence : Vec<Instruction> = Vec::new();

        loop {
            let byte = self.byte_iterator.next();

            match byte {
                Some(valid_byte) => {
                    let instruction: Instruction = match Instruction::try_from(valid_byte.unwrap()) {
                        Ok(valid_instruction) => valid_instruction,
                        Err(e) => { println!("Recovering error: {}", e); continue }
                    };

                    match instruction {
                        Instruction::JumpForward => {
                            let mut sub_sequence = self.hold_sequence();
                            sequence.append(&mut sub_sequence);
                        },
                        Instruction::JumpBackward => break,
                        _ => sequence.push(instruction)
                    }
                },
                None => println!("Invalid byte value in sequence")
            }
        }

        sequence
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


        // 1. Record data pointer value
        // 2. Hold sequence until matching ] (recursive)
        //    - 3 options: We find a matching ] and program is well formed
        //    - We find a ] after another [ (nested subsequence)
        //    - We reach the end of the program and issue an error
        // 3. Perform operation using sequence

        let byte_value = self.cells[self.data_pointer];
        let mut instruction_sequence : Vec<Instruction> = Vec::new();

        // Perform operations if necessary or ignore sequence
        if byte_value != 0 {
            self.interpret_sequence(instruction_sequence);
        }
    }

    fn jump_backward(&mut self) {
        // if the byte at the data pointer is nonzero,
        // then instead of moving the instruction pointer forward to the next command,
        // jump it back to the command after the matching [ command.

        // Two options, either we're in a sequence, or no matching [
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
    let buf_reader: BufReader<File> = BufReader::new(file);
    let mut program = Program{  cells, data_pointer, byte_iterator: buf_reader.bytes() };

    interpret_program(&mut program);

    Ok(())
}

fn interpret_program(program: &mut Program) {
    use instruction::Instruction;

    loop {
        let byte = program.byte_iterator.next();

        match byte {
            Some(valid_byte) => {
                let instruction: Instruction = match Instruction::try_from(valid_byte.unwrap()) {
                    Ok(valid_instruction) => valid_instruction,
                    Err(e) => { println!("Recovering error: {}", e); continue }
                };

                // TODO: Do stuff here
            }
            None => { println!("Compiling done!"); break }
        }
    }
}