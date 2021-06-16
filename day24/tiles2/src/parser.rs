use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;

use crate::instructions::{Direction, InstructionSet, PuzzleInput};

pub fn puzzle_input(input: &str) -> IResult<&str, PuzzleInput> {
    map(
        separated_list1(line_ending, instruction_set),
        |instruction_sets| PuzzleInput::new(instruction_sets)
    )(input)
}

fn instruction_set(input: &str) -> IResult<&str, InstructionSet> {
    map(
        many1(direction),
        |directions| InstructionSet::new(directions)
    )(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((west, northwest, northeast, east, southeast, southwest))(input)
}

fn west(input: &str) -> IResult<&str, Direction> {
    map(tag("w"), |_| Direction::West)(input)
}

fn northwest(input: &str) -> IResult<&str, Direction> {
    map(tag("nw"), |_| Direction::Northwest)(input)
}

fn northeast(input: &str) -> IResult<&str, Direction> {
    map(tag("ne"), |_| Direction::Northeast)(input)
}

fn east(input: &str) -> IResult<&str, Direction> {
    map(tag("e"), |_| Direction::East)(input)
}

fn southeast(input: &str) -> IResult<&str, Direction> {
    map(tag("se"), |_| Direction::Southeast)(input)
}

fn southwest(input: &str) -> IResult<&str, Direction> {
    map(tag("sw"), |_| Direction::Southwest)(input)
}