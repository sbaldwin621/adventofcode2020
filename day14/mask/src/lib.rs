use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use program::{Instruction, Program};

pub mod config;
mod program;

pub fn run(config: Config) -> Result<u64, Box<dyn Error>> {
    let filename = config.filename;

    let mut instructions = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines {
        let instruction = line?.parse::<Instruction>()?;
        instructions.push(instruction);
    }

    let program = Program::new(instructions);
    
    let sum = program.execute();

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
