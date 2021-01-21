use multi::separated_list0c;
use nom::{IResult, multi};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::{digit1, line_ending, space0, space1, satisfy};
use nom::combinator::{map, map_res};
use nom::error::ParseError;
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, tuple};

use crate::validator::{PuzzleInput, Rule, Ruleset};

pub(crate) fn puzzle_input(input: &str) -> IResult<&str, PuzzleInput> {
  map(
    tuple((ruleset, line_ending, line_ending, received_messages)),
    |(ruleset, _, _, received_messages)| PuzzleInput::new(ruleset, received_messages)
  )(input)
}

fn ruleset(input: &str) -> IResult<&str, Ruleset> {
  map(
    separated_list0(line_ending, ruleset_line),
    |lines| Ruleset::from(lines)
  )(input)
}

fn ruleset_line(input: &str) -> IResult<&str, (u64, Rule)> {
  let (input, (line_number, _, rule)) = tuple((number, ws(tag(":")), rule))(input)?;

  Ok((input, (line_number, rule)))
}

fn rule(input: &str) -> IResult<&str, Rule> {
  alt((
    character_match,
    reference_expr
  ))(input)
}

fn character_match(input: &str) -> IResult<&str, Rule> {
  map(
    ws(delimited(tag("\""), satisfy(|c| c.is_alphabetic()), tag("\""))),
    |c| Rule::Character(c)
  )(input)
}

fn reference_expr(input: &str) -> IResult<&str, Rule> {
  map(
    separated_list1(or, and_expr),
    |rules| Rule::Or(rules)
  )(input)
}

fn and_expr(input: &str) -> IResult<&str, Rule> {
  map(
    separated_list1(space1, number),
    |numbers| Rule::And(numbers.iter().map(|n| Rule::Reference(*n)).collect::<Vec<_>>())
  )(input)
}

fn number(input: &str) -> IResult<&str, u64> {
  map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

fn or(input: &str) -> IResult<&str, &str> {
  ws(tag("|"))(input)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    space0,
    inner,
    space0
  )
}

fn received_messages(input: &str) -> IResult<&str, Vec<String>> {
  map(
    separated_list0(line_ending, take_while1(|c: char| c.is_alphabetic())),
    |lines: Vec<&str>| lines.iter().map(|l| l.to_string()).collect::<Vec<String>>()
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    let result = ruleset_line("1: 2 3 | 3 2");

    println!("{:?}", result);
  }

  #[test]
  fn example_ruleset() {
    let (_, result) = ruleset("0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"").unwrap();

    println!("{:?}", result);

    assert!(result.validate("ababbb"));
    assert!(result.validate("abbbab"));
    assert!(!result.validate("bababa"));
    assert!(!result.validate("aaabbb"));
    assert!(!result.validate("aaaabbb"));
  }
}