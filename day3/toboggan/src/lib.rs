use std::{error::Error, fmt::Display, str::FromStr};
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;
    let horizontal_speed = config.horizontal_speed;

    let mut count = 0;
    let mut x = 0;

    let lines = read_lines(filename)?;
    for line in lines {
        let latitude = line?.parse::<Latitude>()?;

        let i = x % latitude.trees.len();
        let tree_at_i = latitude.trees[i];
        if tree_at_i {
            count = count + 1;
        }

        x = x + horizontal_speed;
    }

    Ok(count)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Latitude {
    trees: Vec<bool>
}

impl FromStr for Latitude {
    type Err = ParseLatitudeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ..##.......
        let trees: Vec<bool> = s.chars().map(|c| { 
            match c {
                '#' => true,
                _ => false
            }
        }).collect();

        Ok(Latitude { trees })
    }
}

#[derive(Debug)]
struct ParseLatitudeError { }

impl Display for ParseLatitudeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "couldn't parse latitude")
    }
}

impl Error for ParseLatitudeError { }

pub struct Config {
    pub filename: String,
    pub horizontal_speed: usize,
    pub vertical_speed: usize
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 4 {
            return Err(Box::new(ParseConfigError { kind: ParseConfigErrorKind::NotEnoughArguments }));
        }

        let filename = args[1].clone();
        let horizontal_speed = args[2].parse::<usize>()?;
        let vertical_speed = args[3].parse::<usize>()?;
    
        Ok(Config { filename, horizontal_speed, vertical_speed })
    }
}

#[derive(Debug)]
struct ParseConfigError {
    kind: ParseConfigErrorKind
}

impl Display for ParseConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match &self.kind {
            ParseConfigErrorKind::NotEnoughArguments => "not enough arguments"
        })
    }
}

impl Error for ParseConfigError { }

#[derive(Debug)]
enum ParseConfigErrorKind {
    NotEnoughArguments
}