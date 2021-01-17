use std::ops::Range;

use nom::branch::alt;
use nom::combinator::{eof, map_res};
use nom::IResult;
use nom::bytes::complete::{tag, take_while};
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated, tuple};

use crate::notes::{Notes, Rule, Ruleset, Ticket, TicketSet};

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while(is_alpha)(input)
}

fn is_alpha(chr: char) -> bool {
    chr.is_ascii_lowercase() || chr == ' '
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(take_while(is_digit), |s: &str| s.parse::<u64>())(input)
}

fn is_digit(chr: char) -> bool {
    chr.is_ascii_digit()
}

fn eol(input: &str) -> IResult<&str, &str> {
    alt((tag("\n"), tag("\r\n"), tag("\r")))(input)
}

fn range(input: &str) -> IResult<&str, Range<u64>> {
    let (input, (start, end)) = separated_pair(number, tag("-"), number)(input)?;
    let range = Range { start, end: end + 1 }; // end + 1 because Range is exclusive on top end
    
    Ok((input, range))
}

fn range_list(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    separated_list1(tag(" or "), range)(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, (name, _, ranges, _)) = tuple((
        identifier,
        tag(": "),
        range_list,
        eol
    ))(input)?;

    Ok((input, Rule::new(name, ranges)))
}

fn ruleset(input: &str) -> IResult<&str, Ruleset> {
    let (input, rules) = many1(rule)(input)?;

    Ok((input, Ruleset::new(rules)))
}

fn ticket(input: &str) -> IResult<&str, Ticket> {
    let (input, values) = terminated(separated_list0(tag(","), number), eol)(input)?;

    Ok((input, Ticket::new(values)))
}

fn my_ticket(input: &str) -> IResult<&str, Ticket> {
    preceded(tuple((tag("your ticket:"), eol)), ticket)(input)
}

fn nearby_tickets(input: &str) -> IResult<&str, Vec<Ticket>> {
    preceded(tuple((tag("nearby tickets:"), eol)), many1(ticket))(input)
}

pub(crate) fn parse_notes(input: &str) -> IResult<&str, Notes> {
    let (input, (ruleset, _, my_ticket, _, nearby_tickets)) = tuple((
        ruleset,
        eol,
        my_ticket,
        eol,
        nearby_tickets
    ))(input)?;

    let nearby_tickets = TicketSet::new(nearby_tickets);
    
    Ok((input, Notes::new(ruleset, my_ticket, nearby_tickets)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (_, result) = ruleset("class: 1-3 or 5-7").unwrap();

        println!("{:?}", result);
    }

    #[test]
    fn full_notes() {
        let (_, result) = parse_notes("class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12").unwrap();

        println!("{:?}", result);
    }
}