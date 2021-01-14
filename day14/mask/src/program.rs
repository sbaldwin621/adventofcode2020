use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

// mask = 0010011010X1000100X101011X10010X1010
// mem[57319] = 8001842
// mem[29943] = 1246
// mem[3087] = 1055661079

pub fn apply_mask(value: u64, high_mask: u64, low_mask: u64) -> u64 {
    let high_masked = value | high_mask;
    let masked = !(!high_masked | low_mask);
    
    masked
}

#[derive(Debug)]
pub enum Instruction {
    Mask { high_mask: u64, low_mask: u64 },
    Mem { address: u64, value: u64 }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static MASK_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"mask = ([0-9X]{36})").unwrap()
        });
        static MEM_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"mem\[\d+\] = (\d+)").unwrap()
        });

        if let Some(capture) = MASK_RE.captures(s) {
            let mut low_mask: u64 = 0;
            let mut high_mask: u64 = 0;

            let mask_str = &capture[1];
            for (i, c) in mask_str.chars().enumerate() {
                if i > 0 {
                    low_mask = low_mask << 1;
                    high_mask = high_mask << 1;
                }

                if c == '0' {
                    low_mask = low_mask | 1;
                } else if c == '1' {
                    high_mask = high_mask | 1;
                }
            }

            return Ok(Instruction::Mask { high_mask, low_mask });
        } else if let Some(capture) = MEM_RE.captures(s) {
            let address_str = &capture[1];
            let value_str = &capture[2];
        } else {
            panic!("unrecognized instruction");
        }

        todo!()
    }
}

#[derive(Debug)]
pub enum ParseInstructionError {
    
}

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
 }

impl Error for ParseInstructionError { }

#[cfg(test)]
mod tests {
    use super::*;

    // mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    // mem[8] = 11
    // mem[7] = 101
    // mem[8] = 0
    #[test]
    fn example_mask() {
        let instruction = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse::<Instruction>().unwrap();
        if let Instruction::Mask { high_mask, low_mask } = instruction {
            println!("high = {:b}, low = {:b}", high_mask, low_mask);

            assert_eq!(73, apply_mask(11, high_mask, low_mask));
            assert_eq!(101, apply_mask(101, high_mask, low_mask));
            assert_eq!(64, apply_mask(0, high_mask, low_mask));

        } else {
            panic!("expected Mask, got {:?}", instruction);
        }
    }
}