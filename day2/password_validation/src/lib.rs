use std::fmt::Display;
use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use once_cell::sync::Lazy;

use regex::Regex;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let mut count: usize = 0;

    let filename = config.filename;

    let lines = read_lines(filename)?;
    for line in lines {
        let parsed = line?.parse::<PasswordEntry>()?;
        
        let matching_character_count = parsed.password
            .chars()
            .filter(|c| c == &parsed.required_character)
            .count();

        if matching_character_count >= parsed.minimum && matching_character_count <= parsed.maximum {
            count = count + 1;
        }
    }

    Ok(count)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct PasswordEntry {
    required_character: char,
    minimum: usize,
    maximum: usize,
    password: String
}

impl FromStr for PasswordEntry {
    type Err = ParsePasswordEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 6-20 n: qlzsnnnndwnlhwnxhvjn
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(\d+)-(\d+) ([a-z]): (.*?)$").unwrap()
        });

        if let Some(captures) = RE.captures(s) {
            let minimum_string = &captures[1];
            let maximum_string = &captures[2];
            let character = &captures[3];
            let password = &captures[4];

            let minimum = minimum_string.parse::<usize>()
                .or_else(|_| Err(ParsePasswordEntryError { }))?;

            let maximum = maximum_string.parse::<usize>()
                .or_else(|_| Err(ParsePasswordEntryError { }))?;

            let required_character = (match character.chars().nth(0) {
                Some(c) => Ok(c),
                None    => Err(ParsePasswordEntryError { })
            })?;

            return Ok(PasswordEntry { required_character, minimum, maximum, password: String::from(password) });
        } else {
            return Err(ParsePasswordEntryError { });
        }
    }
}

#[derive(Debug)]
struct ParsePasswordEntryError { }

impl Error for ParsePasswordEntryError { }

impl Display for ParsePasswordEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "could not parse password entry")
    }
}

pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
    
        Ok(Config { filename })
    }
}

