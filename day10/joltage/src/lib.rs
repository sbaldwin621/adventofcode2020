use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;

    let mut adapters = Vec::new();

    adapters.push(0); // outlet with effective joltage of 0

    let lines = read_lines(filename)?;
    for line in lines {
        let adapter = line?.parse::<usize>()?;
        adapters.push(adapter);
    }

    adapters.sort();

    adapters.push(adapters.last().unwrap() + 3); // device with joltage of 3 higher than highest adapter

    let mut one_count = 0;
    let mut three_count = 0;

    for i in 0..adapters.len() - 1 {
        let adapter = adapters[i];
        let next_adapter = adapters[i + 1];

        match next_adapter - adapter {
            1 => { 
                one_count = one_count + 1;
            },
            3 => {
                three_count = three_count + 1;
            },
            _ => { }
        }
    }

    println!("one: {}, three: {}", one_count, three_count);

    Ok(one_count * three_count)
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
