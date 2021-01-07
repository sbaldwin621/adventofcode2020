use std::{error::Error, fmt::Display};
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use xmas::{find_invalid_number, find_weakness};

mod xmas;

pub fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let filename = config.filename;
    let preamble_size = config.preamble_size;

    let mut data = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines {
        data.push(line?.parse::<i64>()?);
    }

    let invalid_number = find_invalid_number(&data, preamble_size)
        .ok_or(ApplicationError::CouldntFindInvalidNumber)?;

    let weakness = find_weakness(&data, invalid_number)
        .ok_or(ApplicationError::CouldntFindWeakness)?;

    Ok(weakness)
}

#[derive(Debug)]
pub enum ApplicationError {
    CouldntFindInvalidNumber,
    CouldntFindWeakness
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ApplicationError::CouldntFindInvalidNumber => "couldn't find invalid number",
            ApplicationError::CouldntFindWeakness => "couldn't find weakness"
        })
    }
}

impl Error for ApplicationError { }

#[derive(Debug)]
enum ApplicationErrorKind {

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Config {
    pub filename: String,
    pub preamble_size: usize
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ParseConfigError> {
        if args.len() < 3 {
            return Err(ParseConfigError { kind: ParseConfigErrorKind::NotEnoughArguments });
        }

        let filename = args[1].clone();
        let preamble_size = match args[2].parse::<usize>() {
            Ok(v) => v,
            Err(_) => { return Err(ParseConfigError { kind: ParseConfigErrorKind::InvalidPreambleSize }); }
        };
    
        Ok(Config { filename, preamble_size })
    }
}

#[derive(Debug)]
pub struct ParseConfigError {
    kind: ParseConfigErrorKind
}

impl Display for ParseConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self.kind {
            ParseConfigErrorKind::NotEnoughArguments => "not enough arguments",
            ParseConfigErrorKind::InvalidPreambleSize => "couldn't parse preamble size"
        })
    }
}

impl Error for ParseConfigError { }

#[derive(Debug)]
pub enum ParseConfigErrorKind {
    NotEnoughArguments,
    InvalidPreambleSize
}