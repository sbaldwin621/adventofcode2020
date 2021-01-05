use std::{collections::HashSet, error::Error, fmt::Display, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

// light red bags contain 1 bright white bag, 2 muted yellow bags.
// faded blue bags contain no other bags.
#[derive(Debug, PartialEq)]
pub struct Rule {
    identifier: String,
    contains: Contains
}

#[derive(Debug, PartialEq)]
pub enum Contains {
    NoOtherBags,
    Some(Vec<(usize, String)>)
}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RULE_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(\w+ \w+) bags contain (.*?)\.$").unwrap()
        });
        static CONTAINS_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^\s*(\d+) (\w+ \w+) bags?$").unwrap()
        });

        if let Some(rule_captures) = RULE_RE.captures(s) {
            let identifier = String::from(&rule_captures[1]);
            let contains_text = &rule_captures[2];

            let contains: Contains;
            if contains_text == "no other bags" {
                contains = Contains::NoOtherBags;
            } else {
                let mut other_bags: Vec<(usize, String)> = Vec::new();
                for contains_clause in contains_text.split(',') {
                    if let Some(contains_captures) = CONTAINS_RE.captures(contains_clause) {
                        let count = match contains_captures[1].parse::<usize>() {
                            Ok(value) => { value }
                            Err(_) => { return Err(ParseRuleError { }); }
                        };

                        let contains_identifier = &contains_captures[2];

                        other_bags.push((count, String::from(contains_identifier)));
                    } else {
                        return Err(ParseRuleError { });
                    }
                }

                contains = Contains::Some(other_bags);
            }

            return Ok(Rule { identifier, contains });
        } else {
            return Err(ParseRuleError { });
        }
    }
}

#[derive(Debug)]
pub struct ParseRuleError { }

impl Display for ParseRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "couldn't parse rule")
    }
}

impl Error for ParseRuleError { }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_rule_with_multiple_bags() {
        let rule = "light red bags contain 1 bright white bag, 2 muted yellow bags.".parse::<Rule>().unwrap();

        assert_eq!(Rule {
            identifier: String::from("light red"),
            contains: Contains::Some(vec![
                (1, String::from("bright white")),
                (2, String::from("muted yellow"))
            ]) }, rule);

        println!("{:?}", rule);
    }

    #[test]
    fn parse_rule_with_no_other_bags() {
        let rule = "faded blue bags contain no other bags.".parse::<Rule>().unwrap();

        assert_eq!(Rule {
            identifier: String::from("faded blue"),
            contains: Contains::NoOtherBags
        }, rule);

        println!("{:?}", rule);
    }
}