use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::ops::{Index, Range};
use std::slice::Iter;
use std::str::FromStr;

use nom::bitvec::index;

use crate::parser::parse_notes;
#[derive(Debug)]
pub struct Notes {
    ruleset: Ruleset,
    my_ticket: Ticket,
    nearby_tickets: TicketSet
}

impl Notes {
    pub fn new(ruleset: Ruleset, my_ticket: Ticket, nearby_tickets: TicketSet) -> Notes {
        Notes { ruleset, my_ticket, nearby_tickets }
    }

    pub fn ruleset(&self) -> &Ruleset {
        &self.ruleset
    }

    pub fn my_ticket(&self) -> &Ticket {
        &self.my_ticket
    }

    pub fn get_valid_nearby_tickets(&self) -> TicketSet {
        let mut valid_nearby_tickets = Vec::new();

        for ticket in self.nearby_tickets.iter() {
            if self.ruleset.validate_ticket(&ticket).len() == 0 {
                valid_nearby_tickets.push(ticket.clone());
            }
        }

        TicketSet::new(valid_nearby_tickets)
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

        for value in ticket.iter() {
            if !self.contains(value) {
                result.push(*value);
            }
        }

        result
    }

    pub fn determine_fields(&self, ticket_set: &TicketSet) -> Vec<Field> {
        let mut fields_by_index: HashMap<usize, HashSet<String>> = HashMap::new();
        let mut fields_by_name: HashMap<String, HashSet<usize>> = HashMap::new();

        let column_set = ticket_set.invert();
        for (i, column) in column_set.iter().enumerate() {
            for rule in self.rules.iter() {
                if rule.contains_column(column) {
                    let rule_name = rule.name.clone();
                    if let Some(names_for_index) = fields_by_index.get_mut(&i) {
                        names_for_index.insert(rule_name);
                    } else {
                        let mut names_for_index = HashSet::new();
                        names_for_index.insert(rule_name);

                        fields_by_index.insert(i, names_for_index);
                    }

                    if let Some(indexes_for_name) = fields_by_name.get_mut(&rule.name) {
                        indexes_for_name.insert(i);
                    } else {
                        let mut indexes_for_name = HashSet::new();
                        indexes_for_name.insert(i);

                        fields_by_name.insert(rule.name.clone(), indexes_for_name);
                    }
                }
            }
        }

        let mut fields = Vec::new();

        loop {
            let mut should_continue = false;

            for (index, names_for_index) in fields_by_index.iter_mut() {
                if names_for_index.len() == 1 {
                    let name = names_for_index.iter().nth(0).unwrap().clone();
                    names_for_index.remove(&name);

                    fields.push(Field { name, index: *index });

                    should_continue = true;
                }
            }

            for field in fields.iter() {
                fields_by_index.remove(&field.index);
                fields_by_name.remove(&field.name);
            }

            for (name, indexes_for_name) in fields_by_name.iter_mut() {
                for field in fields.iter() {
                    indexes_for_name.remove(&field.index);
                }

                if indexes_for_name.len() == 1 {
                    let index = *indexes_for_name.iter().nth(0).unwrap();
                    indexes_for_name.remove(&index);

                    fields.push(Field { name: name.clone(), index });

                    should_continue = true;
                }              
            }
            
            for field in fields.iter() {
                fields_by_index.remove(&field.index);
                fields_by_name.remove(&field.name);
            }

            if !should_continue {
                break;
            }
        }
        
        fields
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

    pub fn contains_column(&self, column: &Column) -> bool {
        for value in column.iter() {
            if !self.contains(value) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
pub struct TicketSet {
    tickets: Vec<Ticket>
}

impl TicketSet {
    pub fn new(tickets: Vec<Ticket>) -> TicketSet {
        TicketSet { tickets }
    }

    pub fn iter(&self) -> Iter<'_, Ticket> {
        self.tickets.iter()
    }

    pub fn invert(&self) -> ColumnSet { 
        let mut columns = (0..self.tickets[0].len()).map(|_| Vec::new()).collect::<Vec<_>>();
        
        for ticket in self.tickets.iter() {
            if ticket.len() != columns.len() { 
                panic!("uneven tickets");
            }

            for i in 0..ticket.len() {
                let value = ticket[i];
                let column = &mut columns[i];

                column.push(value);
            }
        }

        let mut result = Vec::new();
        for column in columns {
            result.push(Column { values: column });
        }

        ColumnSet { columns: result }   
    }
}

#[derive(Debug, Clone)]
pub struct Ticket {
    values: Vec<u64>
}

impl Ticket {
    pub fn new(values: Vec<u64>) -> Ticket {
        Ticket { values }
    }

    pub fn get(&self, index: usize) -> Option<&u64> {
        self.values.get(index)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> Iter<u64> {
        self.values.iter()
    }
}

impl Index<usize> for Ticket {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

#[derive(Debug)]
pub struct ColumnSet {
    columns: Vec<Column>
}

impl ColumnSet {
    pub fn new(columns: Vec<Column>) -> ColumnSet {
        ColumnSet { columns }
    }

    pub fn iter(&self) -> Iter<Column> {
        self.columns.iter()
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    values: Vec<u64>
}

impl Column {
    pub fn new(values: Vec<u64>) -> Column {
        Column { values }
    }

    pub fn iter(&self) -> Iter<u64> {
        self.values.iter()
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub index: usize
}

impl Field {
    pub fn new(name: String, index: usize) -> Field {
        Field { name, index }
    }
}