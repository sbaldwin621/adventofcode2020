use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use cups::CupSet;

pub mod config;
pub mod cups;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let puzzle_input = config.puzzle_input;
    
    let mut cup_set = puzzle_input.parse::<CupSet>().unwrap();

    for n in 0..10_000_000 {
        if n % 100_000 == 0 {
            println!("{}", n);
        }

        cup_set.step();
    }

    println!("{:?}", cup_set.answer());

    Ok(0)
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
