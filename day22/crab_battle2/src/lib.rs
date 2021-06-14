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

    for _ in 0..1000 {
        match input.step() {
            game::GameStatus::Continuing => println!("Continuing {:?}", input),
            game::GameStatus::WinnerFound(winning_score) => {
                println!("Winner found {}", winning_score);
                return Ok(winning_score)
            }
        }


    }

    Ok(0)
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
