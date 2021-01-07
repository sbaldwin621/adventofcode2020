use std::{collections::HashSet, error::Error, fmt::Display};
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

mod xmas;

pub fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let filename = config.filename;
    let preamble_size = config.preamble_size;

    let mut data = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines {
        data.push(line?.parse::<i64>()?);
    }

    for i in preamble_size..data.len() {
        if !is_valid(&data, preamble_size, i) {
            return Ok(data[i]);
        }
    }

    Ok(0)
}

fn is_valid(data: &Vec<i64>, preamble_size: usize, i: usize) -> bool {
    let mut set = HashSet::new();

    let num = data[i];
    for j in (i - preamble_size)..i {
        let other = data[j];
        let compliment = num - other;
        
        if set.contains(&compliment) {
            return true;
        } else {
            set.insert(other);
        }
    }
    
    false
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