use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct InstructionSet {
    instructions: Vec<Instruction>
}

impl InstructionSet {
    fn new(instructions: Vec<Instruction>) -> InstructionSet {
        InstructionSet { instructions }
    }

    pub fn get_seat(&self) -> Result<Seat, SeatError> {
        let mut bottom_range = 0;
        let mut top_range = 127;

        let mut left_range = 0;
        let mut right_range = 7;

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::F => {
                    if top_range == bottom_range {
                        return Err(SeatError { kind: SeatErrorKind::TooManyRowInstructions});
                    }

                    top_range = bottom_range + (top_range - bottom_range) / 2;
                },
                Instruction::B => { 
                    if top_range == bottom_range {
                        return Err(SeatError { kind: SeatErrorKind::TooManyRowInstructions});
                    }

                    bottom_range = top_range - (top_range - bottom_range) / 2;
                },
                Instruction::L => { 
                    if left_range == right_range {
                        return Err(SeatError { kind: SeatErrorKind::TooManyColumnInstructions});
                    }

                    right_range = left_range + (right_range - left_range) / 2;
                },
                Instruction::R => { 
                    if left_range == right_range {
                        return Err(SeatError { kind: SeatErrorKind::TooManyColumnInstructions});
                    }
                    
                    left_range = right_range - (right_range - left_range) / 2;
                }
            }
        }

        if bottom_range != top_range {
            Err(SeatError { kind: SeatErrorKind::TooFewRowInstructions })
        } else if left_range != right_range {
            Err(SeatError { kind: SeatErrorKind::TooFewColumnInstructions })
        } else {
            Ok(Seat { row: bottom_range, column: left_range })
        }
    }
}

impl FromStr for InstructionSet {
    type Err = ParseInstructionSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();

        for char in s.chars() {
            let instruction = match char {
                'F' => Instruction::F,
                'B' => Instruction::B,
                'L' => Instruction::L,
                'R' => Instruction::R,
                _ => {
                    return Err(ParseInstructionSetError { })
                }
            };

            instructions.push(instruction);
        }

        Ok(InstructionSet::new(instructions))
    }
}

#[derive(Debug)]
pub struct ParseInstructionSetError { }

impl Error for ParseInstructionSetError { }

impl Display for ParseInstructionSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "couldn't parse instruction set")
    }
}

#[derive(Debug)]
pub struct SeatError {
    kind: SeatErrorKind
}

impl Error for SeatError { }

impl Display for SeatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self.kind {
            SeatErrorKind::TooFewRowInstructions => "too few row instructions",
            SeatErrorKind::TooFewColumnInstructions => "too few column instructions",
            SeatErrorKind::TooManyRowInstructions => "too many row instructions",
            SeatErrorKind::TooManyColumnInstructions => "too many column instructions"
        })
    }
}

#[derive(Debug)]
pub enum SeatErrorKind {
    TooFewRowInstructions,
    TooFewColumnInstructions,
    TooManyRowInstructions,
    TooManyColumnInstructions
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    F,
    B,
    L,
    R
}

#[derive(Debug)]
pub struct Seat {
    row: usize,
    column: usize
}

impl Seat {
    pub fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_simple_instruction_set() {
        let instruction_set = "FB".parse::<InstructionSet>().unwrap();

        assert_eq!(vec![Instruction::F, Instruction::B], instruction_set.instructions);
    }

    #[test]
    fn parse_empty_instruction_set() {
        let instruction_set = "".parse::<InstructionSet>().unwrap();

        assert_eq!(0, instruction_set.instructions.len());
    }

    /*
    For example, consider just the first seven characters of FBFBBFFRLR:

    Start by considering the whole range, rows 0 through 127.
    F means to take the lower half, keeping rows 0 through 63.
    B means to take the upper half, keeping rows 32 through 63.
    F means to take the lower half, keeping rows 32 through 47.
    B means to take the upper half, keeping rows 40 through 47.
    B keeps rows 44 through 47.
    F keeps rows 44 through 45.
    The final F keeps the lower of the two, row 44.

    For example, consider just the last 3 characters of FBFBBFFRLR:

    Start by considering the whole range, columns 0 through 7.
    R means to take the upper half, keeping columns 4 through 7.
    L means to take the lower half, keeping columns 4 through 5.
    The final R keeps the upper of the two, column 5.
    So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
    */
    #[test]
    fn fbfbbffrlr() {
        let instruction_set = "FBFBBFFRLR".parse::<InstructionSet>().unwrap();

        let seat = instruction_set.get_seat().unwrap();

        assert_eq!(44, seat.row);
        assert_eq!(5, seat.column);
    }

    /*
    Every seat also has a unique seat ID: multiply the row by 8, then add the column.
    In this example, the seat has ID 44 * 8 + 5 = 357.
    */
    #[test]
    fn seat_id() {
        let seat = Seat { row: 44, column: 5 };
        let id = seat.id();

        assert_eq!(357, id);
    }
}