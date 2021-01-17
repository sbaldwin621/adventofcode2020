use std::ops::Range;

use nom::combinator::map_res;
use nom::IResult;
use nom::bytes::complete::{tag, take_while};
use nom::sequence::tuple;

fn range(input: &str) -> IResult<&str, Range<u64>> {
    let (input, (start, _, end)) = tuple((number, tag("-"), number))(input)?;
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
        let (_, result) = range("1-5").unwrap();

        println!("{:?}", result);
    }
}