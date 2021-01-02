use std::collections::BTreeSet;
use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead};
use std::ops::Bound::{Included};
use std::path::Path;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;

    let entries = read_map(filename)?;

    for entry in entries.iter() {
        let compliment = 2020 - entry;
        for second_entry in entries.range((Included(0), Included(compliment))) {
            let second_compliment = compliment - second_entry;
            if entries.contains(&second_compliment) {
                return Ok(entry * second_entry * second_compliment);
            }
        }
    }
    
    Ok(0)
}

fn read_map<P>(filename: P) -> Result<BTreeSet<u32>, Box<dyn Error>>
where P: AsRef<Path>, {
    let mut entries: BTreeSet<u32> = BTreeSet::new();

    let lines = read_lines(filename)?;
    for line in lines {
        let parsed = line?.parse::<u32>()?;

        entries.insert(parsed);
    }

    Ok(entries)
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

