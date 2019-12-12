mod instruction;
use instruction::Instruction;
use instruction::Instruction::*;

use std::env;
use std::path::Path;
use std::fs::File;

use std::io;
use std::io::Bytes;
use std::io::{BufReader, Read, Write};

use std::convert::TryFrom;

type Sequence = Vec<SequenceElement>;

enum SequenceElement {
    Instruction(Instruction),
    Sequence(Sequence)
}

struct Program {
    cells: Vec<u8>,
    data_pointer: usize,
    byte_iterator: Bytes<BufReader<File>>
}

/// The Program implementation manages the state of the program.
/// Functions implemented allow instructions to be interpreted and the data pointer as well as memory to change.
impl Program {
    /// Begin interpretation of the program
    fn interpret(&mut self) {
        loop {
            match self.fetch_instruction() {
                Some(valid_instruction) => self.interpret_instruction(&valid_instruction),
                None => return
            }
        }
    }

    /// Returns Some(Instruction) if there are still instructions to be interpreted, otherwise None
    fn fetch_instruction(&mut self) -> Option<Instruction> {
        let byte = self.byte_iterator.next();

        match byte {
            Some(valid_byte) => {
                let instruction: Option<Instruction> = match Instruction::try_from(valid_byte.unwrap()) {
                    Ok(valid_instruction) => Some(valid_instruction),
                    Err(e) => { println!("Recovering error: {}", e); self.fetch_instruction() }
                };

                instruction
            },
            None => { println!("Compiling done!"); None }
        }
    }

    /// Dispatch calls to appropriate functions depending on the instruction given as a parameter
    fn interpret_instruction(&mut self, instruction: &Instruction) {
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

    /// Recursive method used to conditionally execute elements of a Sequence
    fn execute_sequence(&mut self, sequence : &Sequence) {
        let mut sequence_iterator = sequence.iter();

        // TODO: Check recursive call can't be used instead of loop
        loop {
            match sequence_iterator.next() {
                Some(sequence_type) => match sequence_type {
                    SequenceElement::Instruction(valid_instruction) => match *valid_instruction {
                        JumpForward => panic! {"Error in sequence building process"},
                        JumpBackward => panic! {"Error in sequence building process"},
                        _ => self.interpret_instruction(valid_instruction)
                    },

                    SequenceElement::Sequence(parenthetic_sequence) => {
                        while self.cells[self.data_pointer] != 0 {
                            self.execute_sequence(&parenthetic_sequence);
                            /*for element in parenthetic_sequence {
                            // TODO: DRY and make recursive maybe?
                            match element {
                                JumpForward => panic!{"Error in sequence building process"},
                                JumpBackward => panic!{"Error in sequence building process"},
                                _ => self.interpret_instruction(element)
                            }
                        }*/
                        }
                    }
                },

                None => return
            }
        }
    }

    /// Increment the data pointer by one (>)
    fn increment_pointer(&mut self) {
        self.data_pointer += 1;
    }

    /// Decrement the data pointer by one (<)
    fn decrement_pointer(&mut self) {
        self.data_pointer -= 1;
    }

    /// Increment the byte at the data pointer by one (+)
    fn increment_value(&mut self) {
        if self.cells.len() < self.data_pointer + 1 {
            self.cells.resize(self.data_pointer + 1, 0);
        }

        self.cells[self.data_pointer] += 1;
    }

    /// Decrement the byte at the data pointer by one (-)
    fn decrement_value(&mut self) {
        if self.cells.len() < self.data_pointer + 1 {
            self.cells.resize(self.data_pointer + 1, 0);
        }

        self.cells[self.data_pointer] -= 1;
    }

    /// Output the byte at the data pointer (,)
    fn output_value(&mut self) {
        io::stdout().write(&[self.cells[self.data_pointer]]).unwrap();
    }

    /// Accept one byte of input, storing its value in the byte at the data pointer (,)
    fn accept_input(&mut self) {
        io::stdin().read(&mut [self.cells[self.data_pointer]]).unwrap();
    }

    /// Recursively matches pairs
    fn jumpto_matching_bracket(&mut self) {
        loop {
            match self.fetch_instruction() {
                Some(valid_instruction) => {
                    match valid_instruction {
                        JumpForward => self.jumpto_matching_bracket(),
                        JumpBackward => return,
                        _ => continue
                    }
                },
                None => { panic!("No matching ]") }
            };
        }
    }

    /// Builds a nested sequence of instructions between two parentheses
    fn build_sequence(&mut self) -> Sequence {
        let mut parenthetic_sequence : Vec<SequenceElement> = Vec::new();

        loop {
            match self.fetch_instruction() {
                Some(valid_instruction) => {
                    match valid_instruction {
                        JumpForward =>  {
                            let nested_sequence : Sequence = self.build_sequence();
                            parenthetic_sequence.push(SequenceElement::Sequence(nested_sequence));
                        },
                        JumpBackward => return parenthetic_sequence,
                        _ => parenthetic_sequence.push(SequenceElement::Instruction(valid_instruction))
                    }
                },
                // End of program has been reached
                None => return parenthetic_sequence
            }
        }
    }

    /// Jump forward if the data pointer is zero ([)
    fn jump_forward(&mut self) {
        let byte_value = self.cells[self.data_pointer];

        // Jump if the byte at the data pointer is zero
        if byte_value == 0 {
            // Jump to instruction after matching ] (closed bracket)
            self.jumpto_matching_bracket();
        } else {
            // Hold sequence
            // Recursively execute and check
            let sequence : Sequence = self.build_sequence();
            while self.cells[self.data_pointer] != 0 {
                self.execute_sequence(&sequence)
            }
        }
    }

    /// Jump backward if the data pointer is nonzero (])
    fn jump_backward(&mut self) {
        // This function should never be called outside a sequence
        panic!("No matching \"[\" open bracket!")
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Filename required");
    }

    // Open the program stored in a file
    let filename = &args[1];
    let path = Path::new(filename);
    let file = File::open(path)?; // ? Means push errors up the stack

    // Initialise 30, 000 8-bit cells at runtime
    let cells: Vec<u8> = Vec::with_capacity(30000);
    let data_pointer: usize = 0;
    let buf_reader: BufReader<File> = BufReader::new(file);

    let mut program = Program{  cells, data_pointer, byte_iterator: buf_reader.bytes() };

    // Begin interpretation of the program
    program.interpret();

    Ok(())
}