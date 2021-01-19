#![feature(box_patterns)]
#![feature(box_syntax)]

use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
// use math::{Term, evaluate};
use parser::term;

pub mod config;
mod math;
mod parser;

pub fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let filename = config.filename;

    let mut sum = 0;

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        let (_, parsed) = term(&line).map_err(|_| ApplicationError::AnError)?;
        let result = parsed.eval();
        println!("{} = {}", line, result);

        sum += result;
    }

    Ok(sum)
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
