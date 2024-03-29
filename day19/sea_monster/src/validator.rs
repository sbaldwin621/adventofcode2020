use std::collections::HashMap;
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
            if self.ruleset.validate(message) {
                valid_count += 1;
            }
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
            if let ValidationResult::Yes(next_i) = self.validate_rule(input, 0, &rule_zero) {
                // must have consumed entire input
                if next_i == input.chars().count() {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            panic!("no rule 0");
        }
    }

    fn validate_rule(&self, input: &str, i: usize, rule: &Rule) -> ValidationResult {
        if i >= input.chars().count() {
            return ValidationResult::No;
        }

        match rule {
            Rule::Character(c) => {
                if let Some(first_char) = input.chars().nth(i) {
                    if first_char == *c {
                        return ValidationResult::Yes(i + 1); // advance a character
                    }
                }

                return ValidationResult::No;
            }
            Rule::Reference(line_number) => {
                if let Some(matching_rule) = self.rules.get(line_number) {
                    self.validate_rule(input, i, matching_rule)
                } else {
                    ValidationResult::Err("no matching rule")
                }
            }
            Rule::And(rules) => {
                let mut i = i;

                for rule in rules.iter() {
                    if let ValidationResult::Yes(next_i) = self.validate_rule(input, i, rule) {
                        i = next_i;
                    } else {
                        return ValidationResult::No;
                    }
                }

                ValidationResult::Yes(i)
            }
            Rule::Or(rules) => {
                for rule in rules.iter() {
                    let result = self.validate_rule(input, i, rule);
                    if let ValidationResult::Yes(_) = result {
                        return result;
                    }
                }

                ValidationResult::No
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

enum ValidationResult {
    Yes(usize),
    No,
    Err(&'static str)
}

#[derive(Debug)]
pub enum Rule {
    Character(char),
    Reference(u64),
    And(Vec<Rule>),
    Or(Vec<Rule>)
}