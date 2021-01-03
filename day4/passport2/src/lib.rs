use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use once_cell::sync::Lazy;
use regex::Regex;

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
            if is_valid_passport(&current_passport) {
                count = count + 1;
                println!("{:?}", current_passport);
            }

            current_passport.clear();
        }
    }

    if is_valid_passport(&current_passport) {
        count = count + 1;
        println!("{:?}", current_passport);
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

fn is_valid_passport(passport: &HashMap<String, String>) -> bool {
    has_valid_birth_year(passport) &&
    has_valid_issue_year(passport) &&
    has_valid_expiration_year(passport) &&
    has_valid_height(passport) &&
    has_valid_hair_color(passport) &&
    has_valid_eye_color(passport) &&
    has_valid_passport_id(passport)
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn has_valid_birth_year(passport: &HashMap<String, String>) -> bool {
    if let Some(birth_year) = passport.get("byr") {
        if is_number_between(&birth_year, 1920, 2002) {
            return true;
        }
    }

    false
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn has_valid_issue_year(passport: &HashMap<String, String>) -> bool {
    if let Some(issue_year) = passport.get("iyr") {
        if is_number_between(&issue_year, 2010, 2020) {
            return true;
        }
    }

    false
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn has_valid_expiration_year(passport: &HashMap<String, String>) -> bool {
    if let Some(expiration_year) = passport.get("eyr") {
        if is_number_between(&expiration_year, 2020, 2030) {
            return true;
        }
    }

    false
}

// hgt (Height) - a number followed by either cm or in:
//     - If cm, the number must be at least 150 and at most 193.
//     - If in, the number must be at least 59 and at most 76.
fn has_valid_height(passport: &HashMap<String, String>) -> bool {
    static HEIGHT: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(\d+)(cm|in)$").unwrap()
    });
    
    if let Some(height) = passport.get("hgt") {
        if let Some(captures) = HEIGHT.captures(height) {
            let number = &captures[1];
            let unit = &captures[2];

            if unit == "cm" {
                if is_number_between(number, 150, 193) {
                    return true;
                }
            } else if unit == "in" {
                if is_number_between(number, 59, 76) {
                    return true;
                }
            } 
        }
    }

    false
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn has_valid_hair_color(passport: &HashMap<String, String>) -> bool {
    static HAIR_COLOR: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^#[0-9a-f]{6}$").unwrap()
    });

    if let Some(hair_color) = passport.get("hcl") {
        if HAIR_COLOR.is_match(hair_color) {
            return true;
        }
    }

    false
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn has_valid_eye_color(passport: &HashMap<String, String>) -> bool {
    static EYE_COLORS: Lazy<HashSet<String>> = Lazy::new(|| {
        let mut set = HashSet::new();
        for color in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter() {
            set.insert(String::from(*color));
        }
        set
    });

    if let Some(eye_color) = passport.get("ecl") {
        if EYE_COLORS.contains(eye_color) {
            return true;
        }
    }

    false
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn has_valid_passport_id(passport: &HashMap<String, String>) -> bool {
    static PASSPORT_ID: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[0-9]{9}$").unwrap()
    });

    if let Some(passport_id) = passport.get("pid") {
        if PASSPORT_ID.is_match(passport_id) {
            return true;
        }
    } 

    false
}

fn is_number_between(value: &str, start: usize, end: usize) -> bool {
    let parse_result = value.parse::<usize>();
    if let Ok(year) = parse_result {
        return year >= start && year <= end;
    } 

    false
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