use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::parser::puzzle_input;

#[derive(Debug)]
pub struct PuzzleInput {
    ruleset: Ruleset,
    messages: Vec<String>
}

impl PuzzleInput {
    pub fn new(ruleset: Ruleset, messages: Vec<String>) -> PuzzleInput {
        PuzzleInput { ruleset, messages }
    }

    pub fn get_valid_count(&self) -> usize {
        let mut valid_count = 0;

        for message in self.messages.iter() {
            let is_valid = self.ruleset.validate(message);
            if is_valid {
                valid_count += 1;
            }

            println!("{} {}", if is_valid { "✓" } else { "x" }, message);
        }

        valid_count
    }
}

impl FromStr for PuzzleInput {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        puzzle_input(s)
            .map(|(_, result)| result)
            .map_err(|_| "failed to parse puzzle input")
    }
}

#[derive(Debug)]
pub struct Ruleset {
    rules: HashMap<u64, Rule>
}

impl Ruleset {
    pub fn new() -> Ruleset {
        let rules = HashMap::new();
        Ruleset { rules }
    }

    pub fn insert(&mut self, line_number: u64, rule: Rule) {
        self.rules.insert(line_number, rule);
    }

    pub fn validate(&self, input: &str) -> bool {
        if let Some(rule_zero) = self.rules.get(&0) {
            let mut input_set = HashSet::new();
            input_set.insert(input);

            if let ValidationResult::Yes(remaining) = self.validate_rule(input_set, &rule_zero, true) {
                // must have consumed entire input
                for possibility in remaining {
                    if possibility.len() == 0 {
                        return true;
                    }
                }

                return false;
            } else {
                false
            }
        } else {
            panic!("no rule 0");
        }
    }

    fn validate_rule<'a>(&self, input_set: HashSet<&'a str>, rule: &Rule, expect_complete: bool) -> ValidationResult<'a> {
        // println!("{:?} on {:?} (expect_complete: {})", rule, input_set, expect_complete);

        match rule {
            Rule::Character(c) => {
                let mut result = HashSet::new();

                for input in input_set {
                    if let Some(first_char) = input.chars().nth(0) {
                        if first_char == *c {
                            result.insert(&input[1..]);
                        }
                    }
                }

                if result.len() > 0 || expect_complete {
                    return ValidationResult::Yes(result);
                }
                
                return ValidationResult::No;
            }
            Rule::Reference(line_number) => {
                if let Some(matching_rule) = self.rules.get(line_number) {
                    self.validate_rule(input_set, matching_rule, expect_complete)
                } else {
                    panic!("no matching rule");
                }
            }
            Rule::And(rules) => {
                let mut and_input_set = input_set;

                for (i, rule) in rules.iter().enumerate() {
                    let expect_complete = expect_complete && i == rules.len() - 1;

                    if let ValidationResult::Yes(remaining) = self.validate_rule(and_input_set, rule, expect_complete) {
                        and_input_set = remaining;
                    } else {
                        return ValidationResult::No;
                    }
                }

                // println!("RESULT: {:?} -> {:?}", rule, and_input_set);

                if and_input_set.len() > 0 {
                    return ValidationResult::Yes(and_input_set);
                } else {
                    return ValidationResult::No;
                }
            }
            Rule::Or(rules) => {
                let mut result = HashSet::new();

                for rule in rules.iter() {
                    if let ValidationResult::Yes(remaining) = self.validate_rule(input_set.clone(), rule, expect_complete) {
                        result.extend(remaining);
                    }
                }

                // println!("RESULT: {:?} on {:?} -> {:?}", rule, input_set, result);

                if result.len() > 0 {
                    return ValidationResult::Yes(result);
                } else {
                    return ValidationResult::No;
                }
            }
        }
    }
}

impl From<Vec<(u64, Rule)>> for Ruleset {
    fn from(lines: Vec<(u64, Rule)>) -> Self {
        let mut ruleset = Ruleset::new();

        for (line_number, rule) in lines {
            ruleset.insert(line_number, rule);
        }

        ruleset
    }
}

enum ValidationResult<'a> {
    Yes(HashSet<&'a str>),
    No
}

#[derive(Debug)]
pub enum Rule {
    Character(char),
    Reference(u64),
    And(Vec<Rule>),
    Or(Vec<Rule>)
}

impl Rule {
    pub fn new_character(c: char) -> Rule {
        Rule::Character(c)
    }

    pub fn new_reference(n: u64) -> Rule {
        Rule::Reference(n)
    }

    pub fn new_and(rules: Vec<Rule>) -> Rule {
        if rules.len() == 1 {
            let mut rules = rules;
            rules.remove(0)
        } else {
            Rule::And(rules)
        }
    }

    pub fn new_or(rules: Vec<Rule>) -> Rule {
        let mut rules = rules;
        if rules.len() == 1 {
            let mut rules = rules;
            rules.remove(0)
        } else {
            Rule::Or(rules)
        }
    }
}