use std::ops::Range;

#[derive(Debug)]
pub struct Document {
    ruleset: Ruleset
}

#[derive(Debug)]
pub struct Ruleset {
    rules: Vec<Rule>
}

impl Ruleset {
    pub fn new(rules: Vec<Rule>) -> Ruleset {
        Ruleset { rules }
    }
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    ranges: Vec<Range<usize>>
}

impl Rule {
    pub fn new(name: String, ranges: Vec<Range<usize>>) -> Rule {
        Rule { name, ranges }
    }
}