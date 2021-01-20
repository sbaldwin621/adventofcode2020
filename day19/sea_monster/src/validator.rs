use std::collections::HashMap;

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
        println!("evaluating {:?} ({} @ {})", rule, input, i);

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