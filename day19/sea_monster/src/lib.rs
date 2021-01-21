use std::error::Error;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use parser::puzzle_input;
use validator::PuzzleInput;

pub mod config;
mod parser;
mod validator;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;

    let input = read_to_string(filename)?;
    let input = input.parse::<PuzzleInput>()?;

    let count = input.get_valid_count();

    Ok(count)
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
