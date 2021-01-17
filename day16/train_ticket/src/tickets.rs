use std::ops::Range;

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
    ranges: Vec<Range<u64>>
}

impl Rule {
    pub fn new(name: &str, ranges: Vec<Range<u64>>) -> Rule {
        Rule { name: name.to_string(), ranges }
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
}
