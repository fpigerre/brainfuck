use std::fmt;
use std::convert::TryFrom;

const INCREMENT_POINTER: u8 = 0x3E;
const DECREMENT_POINTER: u8 = 0x3C;
const INCREMENT_VALUE: u8 = 0x2B;
const DECREMENT_VALUE: u8 = 0x2D;
const OUTPUT_VALUE: u8 = 0x2E;
const ACCEPT_INPUT: u8 = 0x2C;
const JUMP_FORWARD: u8 = 0x5B;
const JUMP_BACKWARD: u8 = 0x5D;

/// A simple enum describing available brainfuck instructions
pub enum Instruction {
    IncrementPointer = INCREMENT_POINTER as isize,
    DecrementPointer = DECREMENT_POINTER as isize,
    IncrementValue = INCREMENT_VALUE as isize,
    DecrementValue = DECREMENT_VALUE as isize,
    OutputValue = OUTPUT_VALUE as isize,
    AcceptInput = ACCEPT_INPUT as isize,
    JumpForward = JUMP_FORWARD as isize,
    JumpBackward = JUMP_BACKWARD as isize
}

/// Format instructions for display using symbols
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Instruction::*;

        let symbol = match self {
            IncrementPointer => '>',
            DecrementPointer => '<',
            IncrementValue => '+',
            DecrementValue => '-',
            OutputValue => '.',
            AcceptInput => ',',
            JumpForward => '[',
            JumpBackward => ']'
        };

        write!(f, "{}", symbol)
    }
}

/// For instructions for debugging using instruction names
impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Instruction::*;

        let instruction_name = match self {
            IncrementPointer => "IncrementPointer",
            DecrementPointer => "DecrementPointer",
            IncrementValue => "IncrementValue",
            DecrementValue => "DecrementValue",
            OutputValue => "OutputValue",
            AcceptInput => "AcceptInput",
            JumpForward => "JumpForward",
            JumpBackward => "JumpBackward"
        };

        write!(f, "{}", instruction_name)
    }
}

/// Implementation to convert hexadecimal values to enumerated Instruction values
impl TryFrom<u8> for Instruction {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Instruction::*;
        match value {
            INCREMENT_POINTER => Ok(IncrementPointer),
            DECREMENT_POINTER => Ok(DecrementPointer),
            INCREMENT_VALUE => Ok(IncrementValue),
            DECREMENT_VALUE => Ok(DecrementValue),
            OUTPUT_VALUE => Ok(OutputValue),
            ACCEPT_INPUT => Ok(AcceptInput),
            JUMP_FORWARD => Ok(JumpForward),
            JUMP_BACKWARD => Ok(JumpBackward),
            _ => Err("Unrecognised instruction")
        }
    }
}
