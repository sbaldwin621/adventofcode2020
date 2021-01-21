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
            if let ValidationResult::Yes(remaining) = self.validate_rule(input, &rule_zero) {
                // must have consumed entire input
                if remaining.len() == 0 {
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

    fn validate_rule<'a>(&self, input: &'a str, rule: &Rule) -> ValidationResult<'a> {
        println!("{:?} on \"{}\"", rule, input);

        match rule {
            Rule::Character(c) => {
                if let Some(first_char) = input.chars().nth(0) {
                    if first_char == *c {
                        return ValidationResult::Yes(&input[1..]); // advance a character
                    }
                }

                return ValidationResult::No;
            }
            Rule::Reference(line_number) => {
                if let Some(matching_rule) = self.rules.get(line_number) {
                    self.validate_rule(input, matching_rule)
                } else {
                    ValidationResult::Err("no matching rule")
                }
            }
            Rule::And(rules) => {
                let mut input = input;

                for rule in rules.iter() {
                    if let ValidationResult::Yes(remaining) = self.validate_rule(input, rule) {
                        input = remaining;
                    } else {
                        return ValidationResult::No;
                    }
                }

                ValidationResult::Yes(input)
            }
            Rule::Or(rules) => {
                for rule in rules.iter() {
                    let result = self.validate_rule(input, rule);
                    if let ValidationResult::Yes(remaining) = result {
                        if remaining.len() == 0 {
                            return result;
                        }
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

enum ValidationResult<'a> {
    Yes(&'a str),
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