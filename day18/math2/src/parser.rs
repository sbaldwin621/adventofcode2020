use std::ops::Mul;

use nom::{IResult, multi};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::multispace0;
use nom::combinator::{map, map_res};
use nom::error::ParseError;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded};

use crate::math::{Term};

pub fn term(input: &str) -> IResult<&str, Term> {
    multiplication(input)
}

pub fn top(input: &str) -> IResult<&str, Term> {
    alt((number, parentheses))(input)
}

pub fn addition(input: &str) -> IResult<&str, Term> {
    map(
        pair(
            top,
            many0(preceded(ws(tag("+")), top))
        ),
        |(first, rest)| {
            rest.into_iter().fold(first, |a, b| match b {
                _ => Term::Add(box a, box b)
            })
        }
    )(input)
}

pub fn multiplication(input: &str) -> IResult<&str, Term> {
    map(
        pair(
            addition,
            many0(preceded(ws(tag("*")), addition))
        ),
        |(first, rest)| {
            rest.into_iter().fold(first, |a, b| match b {
                _ => Term::Multiply(box a, box b)
            })
        }
    )(input)
}

fn parentheses(input: &str) -> IResult<&str, Term> {
    map(
        ws( delimited(tag("("), term, tag(")"))),
        |term| Term::Parentheses(box term)
    )(input)
}

fn number(input: &str) -> IResult<&str, Term> {
    map_res(
        ws(take_while(is_digit)), 
        |s: &str| s.parse::<i64>().map(|n| Term::Number(n))
    )(input)
}

fn is_digit(chr: char) -> bool {
    chr.is_ascii_digit()
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example0() {
        let (_, t) = term("1 + 2 + 3").unwrap();

        println!("{:?}", t);

        let result = t.eval();

        assert_eq!(6, result);
    }

    #[test]
    fn example1() {
        let (_, t) = term("1 + 2 * 3 + 4 * 5 + 6").unwrap();

        println!("{:?}", t);

        let result = t.eval();

        assert_eq!(231, result);
    }

    #[test]
    fn example2() {
        let (_, t) = term("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap();

        println!("{:?}", t);

        let result = t.eval();

        assert_eq!(1445, result);
    }

    #[test]
    fn example4() {
        let (_, t) = term("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();

        println!("{:?}", t);

        let result = t.eval();

        assert_eq!(23340, result);
    }

    // #[test]
    // fn example2() {
    //     let (_, terms) = term("1 + (2 * 3) + (4 * (5 + 6))").unwrap();
    //     let result = evaluate(&terms);

    //     assert_eq!(51, result);
    // }
}