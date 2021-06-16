use std::error::Error;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use parser::puzzle_input;

use crate::lobby::Lobby;

pub mod config;
pub mod instructions;
pub mod lobby;
pub mod parser;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;

    let input = read_to_string(filename)?;

    let (_, puzzle_input) = puzzle_input(&input)
        .map_err(|_| ApplicationError::AnError)?;

    let mut lobby = Lobby::new();

    for instruction_set in puzzle_input.instruction_sets.iter() {
        lobby.follow_instructions(instruction_set);
    }

    Ok(lobby.count())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
