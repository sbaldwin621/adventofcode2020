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
    
    let result = notes.get_error_rate();
    
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
