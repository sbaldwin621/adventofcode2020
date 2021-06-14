use std::error::Error;
use std::fmt::Display;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use game::PuzzleInput;

pub mod config;
pub mod game;
pub mod parser;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;

    let input = read_to_string(filename)?;
    let mut input = input.parse::<PuzzleInput>()?;

    println!("{:?}", input);
    
    match input.play() {
        game::GameResult::Player1(winning_score) => Ok(winning_score),
        game::GameResult::Player2(winning_score) => Ok(winning_score),
    }
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
