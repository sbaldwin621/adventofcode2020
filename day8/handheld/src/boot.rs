use std::{collections::HashSet, error::Error, fmt::Display, str::FromStr};

pub fn run_program(program: &Vec<Instruction>) -> isize {
    let mut accum = 0;
    let mut program_counter: isize = 0;

    let mut executed_lines = HashSet::new();

    loop {
        if executed_lines.contains(&program_counter) {
            break;
        }

        executed_lines.insert(program_counter);
        let instruction = program.get(program_counter as usize).unwrap();

        match instruction {
            Instruction::Noop => {
                program_counter = program_counter + 1;
            },
            Instruction::Acc(value) => { 
                accum = accum + value; 
                program_counter = program_counter + 1;
            },
            Instruction::Jump(value) => {
                program_counter = program_counter + value;
            }
        }
    }

    accum
}

pub struct Line {
    line_number: usize,
    instruction: Instruction
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Noop,
    Acc(isize),
    Jump(isize)
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((instruction_string, amount_string)) = s.split_once(' ') {
            if let Ok(amount) = amount_string.parse::<isize>() {
                return match instruction_string {
                    "nop" => { Ok(Instruction::Noop) },
                    "acc" => { Ok(Instruction::Acc(amount)) },
                    "jmp" => { Ok(Instruction::Jump(amount)) },
                    _ => { Err(ParseInstructionError { }) }
                }
            }
        }

        Err(ParseInstructionError { })
    }
}

#[derive(Debug)]
pub struct ParseInstructionError { }

impl Error for ParseInstructionError { }

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "couldn't parse instruction")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acc() {
        let instruction = "acc +1".parse::<Instruction>().unwrap();

        assert_eq!(Instruction::Acc(1), instruction);
    }

    #[test]
    fn nop() {
        let instruction = "nop +1".parse::<Instruction>().unwrap();

        assert_eq!(Instruction::Noop, instruction);
    }

    #[test]
    fn jump() {
        let instruction = "jmp -1".parse::<Instruction>().unwrap();

        assert_eq!(Instruction::Jump(-1), instruction);
    }
}