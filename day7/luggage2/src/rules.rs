use std::{error::Error, fmt::Display, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct RuleSet {
    nodes: Vec<RuleSetNode>,
    edges: Vec<RuleSetEdge>
}

impl RuleSet {
    pub fn new() -> RuleSet {
        RuleSet { nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        let identifier = rule.identifier;

        if let Contains::Some(other_bags) = rule.contains {
            for (count, to) in other_bags {
                self.edges.push(RuleSetEdge { from: identifier.clone(), to, count });
            }
        }
        
        self.nodes.push(RuleSetNode { identifier });
    }

    pub fn count_combinations(&self, target: &str) -> usize {
        let mut count = 0;

        for node in &self.nodes {
            if self.is_valid(&node.identifier, target) {
                count = count + 1;
            }
        }

        count
    }

    fn is_valid(&self, identifier: &str, target: &str) -> bool {
        for edge in &self.edges {
            if edge.from == identifier {
                if edge.to == target {
                    return true;
                } else if self.is_valid(&edge.to, target) {
                    return true;
                }
            }
        }

        false
    }
    
    pub fn count_required(&self, target: &str) -> usize {
        let mut count = 0;

        for edge in &self.edges {
            if edge.from == target {
                count = count + edge.count * (self.count_required(&edge.to) + 1);
            }
        }

        count
    }
}

#[derive(Debug, PartialEq)]
struct RuleSetNode {
    identifier: String
}

#[derive(Debug, PartialEq)]
struct RuleSetEdge {
    from: String,
    to: String,
    count: usize
}

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

    #[test]
    fn example_ruleset() {
        let mut ruleset = RuleSet::new();
        ruleset.add_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("dark orange bags contain 3 bright white bags, 4 muted yellow bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("bright white bags contain 1 shiny gold bag.".parse::<Rule>().unwrap());
        ruleset.add_rule("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("dark olive bags contain 3 faded blue bags, 4 dotted black bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("faded blue bags contain no other bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("dotted black bags contain no other bags.".parse::<Rule>().unwrap());

        let count = ruleset.count_combinations("shiny gold");

        assert_eq!(4, count);
    }

    #[test]
    fn example_ruleset2() {
        let mut ruleset = RuleSet::new();
        ruleset.add_rule("shiny gold bags contain 2 dark red bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("dark red bags contain 2 dark orange bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("dark orange bags contain 2 dark yellow bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("dark yellow bags contain 2 dark green bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("dark green bags contain 2 dark blue bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("dark blue bags contain 2 dark violet bags.".parse::<Rule>().unwrap());
        ruleset.add_rule("dark violet bags contain no other bags.".parse::<Rule>().unwrap());
        
        let count = ruleset.count_required("shiny gold");

        assert_eq!(126, count);
    }
}