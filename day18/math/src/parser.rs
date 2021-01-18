use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{one_of, space0};
use nom::combinator::{map, map_res};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

use crate::math::{Term, Operator};

pub fn term(input: &str) -> IResult<&str, Vec<Term>> {
    map(
        pair(
            alt((
                map(number, |n| Term::Simple(Operator::Add, n)),
                map(parentheses, |p| Term::Nested(Operator::Add, Box::new(p)))
            )),
            many0(alt((
                map(pair(operator, number), |(o, n)| Term::Simple(o, n)),
                map(pair(operator, parentheses), |(o, p)| Term::Nested(o, Box::new(p))),
            )))
        ),
        |(first, rest)| {
            let mut result = vec![first];
            result.extend(rest);

            result
        }
    )(input)
}

fn parentheses(input: &str) -> IResult<&str, Vec<Term>> {
    delimited(space0, delimited(tag("("), term, tag(")")), space0)(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    delimited(space0, map(one_of("+-*"), |s: char| match s {
        '+' => Operator::Add,
        '-' => Operator::Subtract,
        '*' => Operator::Multiply,
        _ => panic!(":(")
    }), space0)(input)
}

fn number(input: &str) -> IResult<&str, i64> {
    delimited(space0, map_res(
        take_while(is_digit), 
        |s: &str| s.parse::<i64>()
    ), space0)(input)
}

fn is_digit(chr: char) -> bool {
    chr.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::evaluate;

    #[test]
    fn example1() {
        let (_, terms) = term("1 + 2 * 3 + 4 * 5 + 6").unwrap();
        let result = evaluate(&terms);

        assert_eq!(71, result);
    }
    #[test]
    fn example2() {
        let (_, terms) = term("1 + (2 * 3) + (4 * (5 + 6))").unwrap();
        let result = evaluate(&terms);

        assert_eq!(51, result);
    }
}