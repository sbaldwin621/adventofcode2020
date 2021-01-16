use std::error::Error;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use game::{NumberGame};

pub mod config;
mod game;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;
    let turns = config.turns;

    let input = read_to_string(filename)?;
    
    let number_game = input.parse::<NumberGame>()?;
    
    let answer = number_game.iter().nth(turns - 1).unwrap();

    Ok(answer)
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
