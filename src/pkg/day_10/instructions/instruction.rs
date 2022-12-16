use std::{fmt, str::FromStr};

use super::{super::cpu::Cpu, addx::Addx, noop::Noop};

pub(crate) trait Instruction {
    fn simulate(&self, cpu: &Cpu) -> Vec<Cpu>;
}

#[derive(Debug)]
pub(crate) struct InstructionParseError;

impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Could not read Instruction.");
    }
}

impl FromStr for Box<dyn Instruction> {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();

        match split[0] {
            "noop" => Ok(Box::new(Noop::new())),
            "addx" => {
                Ok(Box::new(Addx::new(isize::from_str(split[1]).unwrap())))
            }
            _ => Err(InstructionParseError),
        }
    }
}
