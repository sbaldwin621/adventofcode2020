use std::collections::{HashMap};
use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;

    let mut count = 0;
    let mut current_passport = HashMap::new();

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        if line.len() > 0 {
            let pairs = parse_line(&line);
            current_passport.extend(pairs);
        } else {
            if is_passport_valid(&current_passport) {
                count = count + 1;
            }

            current_passport.clear();
        }
    }

    if is_passport_valid(&current_passport) {
        count = count + 1;
    }

    Ok(count)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &String) -> HashMap<String, String> {
    let mut pairs = HashMap::new();

    for pair in line.split(' ') {
        let split_pair = pair.split(':').collect::<Vec<_>>();
        if split_pair.len() == 2 {
            pairs.insert(String::from(split_pair[0]), String::from(split_pair[1]));
        }
    }

    pairs
}

fn is_passport_valid(passport: &HashMap<String, String>) -> bool {
    static EXPECTED_KEYS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for key in EXPECTED_KEYS {
        if !passport.contains_key(*key) {
            return false;
        }
    }

    true
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

