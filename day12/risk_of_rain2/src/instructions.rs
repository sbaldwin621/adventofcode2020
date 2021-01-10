use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

// Action N means to move north by the given value.
// Action S means to move south by the given value.
// Action E means to move east by the given value.
// Action W means to move west by the given value.
// Action L means to turn left the given number of degrees.
// Action R means to turn right the given number of degrees.
// Action F means to move forward by the given value in the direction the ship is currently facing.

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub command: InstructionCommand,
    pub value: isize
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            Err(ParseInstructionError::Empty)
        } else {
            let (first, rest) = s.split_at(1);
            let first_char = first.chars().nth(0).unwrap();

            let command = match first_char {
                'N' => Ok(InstructionCommand::North),
                'S' => Ok(InstructionCommand::South),
                'E' => Ok(InstructionCommand::East),
                'W' => Ok(InstructionCommand::West),
                'L' => Ok(InstructionCommand::Left),
                'R' => Ok(InstructionCommand::Right),
                'F' => Ok(InstructionCommand::Forward),
                _ => Err(ParseInstructionError::InvalidCommand(first_char))
            }?;
            
            let value = rest.parse::<isize>()
                .map_err(|_| ParseInstructionError::InvalidArgument(rest.to_string()))?;
            
            Ok(Instruction { command, value })
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionCommand {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

#[derive(Debug, PartialEq)]
pub enum ParseInstructionError {
    Empty,
    InvalidCommand(char),
    InvalidArgument(String)
}

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseInstructionError::Empty => "cannot parse from an empty string".to_string(),
            ParseInstructionError::InvalidCommand(c) => format!("invalid command '{}'", c),
            ParseInstructionError::InvalidArgument(s) => format!("invalid argument '{}'", s)
        })
    }
}

impl Error for ParseInstructionError { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let command = "N5".parse::<Instruction>().unwrap();

        assert_eq!(Instruction { command: InstructionCommand::North, value: 5 }, command);
    }

    #[test]
    fn parse_empty() {
        let result = "".parse::<Instruction>();

        assert_eq!(Err(ParseInstructionError::Empty), result);
    }
}