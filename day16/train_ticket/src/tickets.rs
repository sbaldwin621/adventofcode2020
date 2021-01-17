use std::ops::Range;

#[derive(Debug)]
pub struct Notes {
    ruleset: Ruleset,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>
}

impl Notes {
}

#[derive(Debug)]
pub struct Ruleset {

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

}
