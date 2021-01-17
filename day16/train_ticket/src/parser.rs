use std::ops::Range;

use nom::combinator::map_res;
use nom::IResult;
use nom::bytes::complete::{tag, take_while};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};

use crate::tickets::Rule;

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, identifier) = identifier(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, ranges) = range_list(input)?;

    Ok((input, Rule::new(identifier, ranges)))
}

fn range_list(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    separated_list1(tag(" or "), range)(input)
}

fn range(input: &str) -> IResult<&str, Range<u64>> {
    let (input, (start, end)) = separated_pair(number, tag("-"), number)(input)?;
    let range = Range { start, end };
    
    Ok((input, range))
}

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while(is_alpha)(input)
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(take_while(is_digit), |s: &str| s.parse::<u64>())(input)
}

fn is_alpha(chr: char) -> bool {
    chr.is_ascii_lowercase()
}

fn is_digit(chr: char) -> bool {
    chr.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (_, result) = rule("class: 1-3 or 5-7").unwrap();

        println!("{:?}", result);
    }
}