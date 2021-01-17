use std::error::Error;
use std::fmt::Display;
use std::fs::read_to_string;

use config::Config;
use notes::Notes;

pub mod config;
mod parser;
mod notes;

pub fn run(config: Config) -> Result<u64, Box<dyn Error>> {
    let filename = config.filename;

    let input = read_to_string(filename)?;
    
    let notes = input.parse::<Notes>()?;
    let ruleset =  notes.ruleset();
    let my_ticket = notes.my_ticket();
    let valid_tickets = notes.get_valid_nearby_tickets();
    let fields = ruleset.determine_fields(&valid_tickets);

    let mut result = 1;

    for field in fields {
        if field.name.starts_with("departure") {
            let corresponding_value = my_ticket.get(field.index).unwrap();
            result = result * corresponding_value;
        }
    }

    Ok(result)
}

#[derive(Debug)]
pub enum ApplicationError {
    AnError
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            AnError => "an error occurred"  
        })
    }
}

impl Error for ApplicationError { }
