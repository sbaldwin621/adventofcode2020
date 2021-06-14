use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::tuple;

use crate::game::Deck;
use crate::game::PuzzleInput;

pub(crate) fn puzzle_input(input: &str) -> IResult<&str, PuzzleInput> {
    map(
        tuple((player1, line_ending, line_ending, player2)),
        |(deck1, _, _, deck2)| PuzzleInput::new(deck1, deck2)
    )(input)
}

fn player1(input: &str) -> IResult<&str, Deck> {
    map(
        tuple((tag("Player 1:"), line_ending, deck)),
        |(_, _, deck)| deck
    )(input)
}

fn player2(input: &str) -> IResult<&str, Deck> {
    map(
        tuple((tag("Player 2:"), line_ending, deck)),
        |(_, _, deck)| deck
    )(input)
}

fn deck(input: &str) -> IResult<&str, Deck> {
    map(
      separated_list0(line_ending, number),
      |lines| Deck::new(lines)
    )(input)
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    let result = puzzle_input("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10");

    println!("{:?}", result);
  }
}