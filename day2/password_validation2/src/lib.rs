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
        
        let mut local_count = 0;

        for position in &[parsed.position_one, parsed.position_two] {
            if let Some(character_one) = parsed.password.chars().nth(position - 1) {
                if character_one == parsed.required_character {
                    local_count = local_count + 1;
                }
            }
        }

        if local_count == 1 {
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
    position_one: usize,
    position_two: usize,
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
            let position_one_string = &captures[1];
            let position_two_string = &captures[2];
            let character = &captures[3];
            let password = String::from(&captures[4]);

            let position_one = position_one_string.parse::<usize>()
                .or_else(|_| Err(ParsePasswordEntryError { }))?;

            let position_two = position_two_string.parse::<usize>()
                .or_else(|_| Err(ParsePasswordEntryError { }))?;

            let required_character = (match character.chars().nth(0) {
                Some(c) => Ok(c),
                None    => Err(ParsePasswordEntryError { })
            })?;

            return Ok(PasswordEntry { required_character, position_one, position_two, password });
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

