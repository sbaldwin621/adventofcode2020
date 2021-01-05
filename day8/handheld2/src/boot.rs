use std::{collections::HashSet, error::Error, fmt::Display, str::FromStr};

pub fn run_program(program: &Vec<Instruction>) -> isize {
    for mutation_index in 0..program.len() {
        if let Some(accum) = does_terminate_with_mutation(program, mutation_index) {
            return accum;
        }
    }

    0
}

fn does_terminate_with_mutation(program: &Vec<Instruction>, mutation: usize) -> Option<isize> {
    let mut accum = 0;
    let mut program_counter: usize = 0;

    let mut executed_lines = HashSet::new();

    loop {
        if program_counter == program.len() {
            println!("finished w/ mutation at {}", mutation);
            return Some(accum);
        } if program_counter > program.len() {
            return None; // Past program end
        } else if executed_lines.contains(&program_counter) {
            return None; // Loop detected
        }

        executed_lines.insert(program_counter);

        let instruction_at_counter = program.get(program_counter as usize).unwrap();
        let flipped_instruction = instruction_at_counter.flip();

        let instruction = if mutation == program_counter {
            &flipped_instruction
        } else {
            instruction_at_counter
        };

        match instruction {
            Instruction::Noop(_) => {
                program_counter = program_counter + 1;
            },
            Instruction::Acc(value) => { 
                accum = accum + value; 
                program_counter = program_counter + 1;
            },
            Instruction::Jump(value) => {
                program_counter = ((program_counter as isize) + value) as usize;
            }
        }
    }
}

pub struct Line {
    line_number: usize,
    instruction: Instruction
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Noop(isize),
    Acc(isize),
    Jump(isize)
}

impl Instruction {
    fn flip(&self) -> Instruction {
        match self {
            Instruction::Noop(value) => { Instruction::Jump(*value) }
            Instruction::Acc(value) => { Instruction::Acc(*value) }
            Instruction::Jump(value) => { Instruction::Noop(*value) }
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((instruction_string, amount_string)) = s.split_once(' ') {
            if let Ok(amount) = amount_string.parse::<isize>() {
                return match instruction_string {
                    "nop" => { Ok(Instruction::Noop(amount)) },
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