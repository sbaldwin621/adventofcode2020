use std::collections::HashSet;
use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;

    let mut pairs: HashSet<u32> = HashSet::new();

    let lines = read_lines(filename)?;
    for line in lines {
        let parsed = line?.parse::<u32>()?;

        if pairs.contains(&parsed) {
            return Ok(parsed * (2020 - parsed));
        } else {
            let compliment = 2020 - parsed;
            pairs.insert(compliment);
        }
    }

    Ok(0)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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

