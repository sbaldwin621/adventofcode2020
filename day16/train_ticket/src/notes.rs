use std::error::Error;
use std::fmt::Display;
use std::ops::Range;
use std::slice::Iter;
use std::str::FromStr;

use crate::parser::parse_notes;
#[derive(Debug)]
pub struct Notes {
    ruleset: Ruleset,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>
}

impl Notes {
    pub fn new(ruleset: Ruleset, my_ticket: Ticket, nearby_tickets: Vec<Ticket>) -> Notes {
        Notes { ruleset, my_ticket, nearby_tickets }
    }

    pub fn get_error_rate(&self) -> u64 {
        let mut result = 0;

        for ticket in self.nearby_tickets.iter() {
            let validation_result = self.ruleset.validate_ticket(ticket);
            
            for invalid_number in validation_result {
                result = result + invalid_number;
            }
        }

        result
    }
}

impl FromStr for Notes {
    type Err = ParseNotesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_notes(s) {
            Ok((_, notes)) => Ok(notes),
            Err(e) => Err(ParseNotesError::InvalidNotes(e.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum ParseNotesError {
    InvalidNotes(String)
}

impl Display for ParseNotesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseNotesError::InvalidNotes(error) => format!("invalid notes: {}", error)
        })
    }
}

impl Error for ParseNotesError { }

#[derive(Debug)]
pub struct Ruleset {
    rules: Vec<Rule>
}

impl Ruleset {
    pub fn new(rules: Vec<Rule>) -> Ruleset {
        Ruleset { rules }
    }

    pub fn validate_ticket(&self, ticket: &Ticket) -> Vec<u64> {
        let mut result = Vec::new();

        for value in ticket.values() {
            if !self.contains(value) {
                result.push(*value);
            }
        }

        result
    }

    fn contains(&self, number: &u64) -> bool {
        for rule in &self.rules {
            if rule.contains(number) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    ranges: Vec<Range<u64>>
}

impl Rule {
    pub fn new(name: &str, ranges: Vec<Range<u64>>) -> Rule {
        Rule { name: name.to_string(), ranges }
    }

    pub fn contains(&self, number: &u64) -> bool {
        for range in &self.ranges {
            if range.contains(&number) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct Ticket {
    values: Vec<u64>
}

impl Ticket {
    pub fn new(values: Vec<u64>) -> Ticket {
        Ticket { values }
    }

    pub fn values(&self) -> Iter<u64> {
        self.values.iter()
    }
}
