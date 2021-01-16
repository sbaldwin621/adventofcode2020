use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

// mask = 0010011010X1000100X101011X10010X1010
// mem[57319] = 8001842
// mem[29943] = 1246
// mem[3087] = 1055661079

pub struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Program {        
        Program { instructions }
    }

    pub fn execute(&self) -> Result<u64, ProgramError> {
        let mut memory: HashMap<u64, u64> = HashMap::new();

        let mut current_mask = None;

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Mask { high_mask, floating_mask } => {
                    current_mask = Some((*high_mask, *floating_mask));
                }
                Instruction::Mem { address, value } => {
                    if let Some((high_mask, floating_mask)) = current_mask {
                        let masked_addresses = apply_mask(*address, high_mask, floating_mask);
                        for address in masked_addresses {
                            memory.insert(address, *value);
                        }
                    } else {
                        return Err(ProgramError::NoMaskSet);   
                    }
                }
            }
        }

        let sum = memory.iter().fold(0, |accum, (_, v)| accum + v);

        Ok(sum)
    }
}

#[derive(Debug)]
pub enum ProgramError {
    NoMaskSet
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ProgramError::NoMaskSet => "no mask set"
        })
    }
}

impl Error for ProgramError { }

#[derive(Debug)]
pub enum Instruction {
    Mask { high_mask: u64, floating_mask: u64 },
    Mem { address: u64, value: u64 }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static MASK_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"mask = ([0-9X]{36})").unwrap()
        });
        static MEM_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap()
        });

        if let Some(capture) = MASK_RE.captures(s) {
            let mut high_mask: u64 = 0;
            let mut floating_mask: u64 = 0;

            let mask_str = &capture[1];
            for (i, c) in mask_str.chars().enumerate() {
                if i > 0 {
                    high_mask = high_mask << 1;
                    floating_mask = floating_mask << 1;
                }

                if c == '1' {
                    high_mask = high_mask | 1;
                } else if c == 'X' {
                    floating_mask = floating_mask | 1;
                }
            }

            return Ok(Instruction::Mask { high_mask, floating_mask });
        } else if let Some(capture) = MEM_RE.captures(s) {
            let address_str = &capture[1];
            let value_str = &capture[2];

            let address = address_str.parse::<u64>().unwrap();
            let value = value_str.parse::<u64>().unwrap();

            return Ok(Instruction::Mem { address, value });
        } else {
            return Err(ParseInstructionError::UnrecognizedCommand);
        }
    }
}

#[derive(Debug)]
pub enum ParseInstructionError {
    UnrecognizedCommand
}

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseInstructionError::UnrecognizedCommand => "unrecognized command"
        })
    }
 }

impl Error for ParseInstructionError { }

fn apply_mask(value: u64, high_mask: u64, floating_mask: u64) -> Vec<u64> {
    let high_masked = value | high_mask;
        
    let mut result = HashSet::new();

    result.insert(high_masked);

    for i in 0..36 {
        let is_active = floating_mask >> i & 1 == 1;
        if is_active {
            let current_mask = 1 << i;
            let inverted_mask = !current_mask;
  
            let mut temp = Vec::new();

            for existing in result.iter() {                     
                let on = existing | current_mask;
                let off = existing & inverted_mask;

                temp.push(on);
                temp.push(off);
            }

            result.extend(temp.iter());

        }
    }

    let mut result = result.into_iter().collect::<Vec<u64>>();
    result.sort();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // mask = 000000000000000000000000000000X1001X
    // mem[42] = 100
    #[test]
    fn example_mask_1() {
        let instruction = "mask = 000000000000000000000000000000X1001X".parse::<Instruction>().unwrap();
        if let Instruction::Mask { high_mask, floating_mask } = instruction {
            let result = apply_mask(42, high_mask, floating_mask);

            assert_eq!(vec![26, 27, 58, 59], result);
        } else {
            panic!("expected Mask, got {:?}", instruction);
        }
    }

    // mask = 00000000000000000000000000000000X0XX
    // mem[26] = 1
    #[test]
    fn example_mask_2() {
        let instruction = "mask = 00000000000000000000000000000000X0XX".parse::<Instruction>().unwrap();
        if let Instruction::Mask { high_mask, floating_mask } = instruction {
            let result = apply_mask(26, high_mask, floating_mask);

            assert_eq!(vec![16, 17, 18, 19, 24, 25, 26, 27], result);
        } else {
            panic!("expected Mask, got {:?}", instruction);
        }
    }
}