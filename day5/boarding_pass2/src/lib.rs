use std::collections::HashSet;
use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

mod instructions;

use instructions::{InstructionSet};

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;

    let mut min_seat_id = 127 * 8 + 7;
    let mut max_seat_id = 0;
    let mut all_seat_ids: HashSet<usize> = HashSet::new();

    for id in 0..=min_seat_id {
        all_seat_ids.insert(id);
    }
    
    let lines = read_lines(filename)?;
    for line in lines {
        let instruction_set = line?.parse::<InstructionSet>()?;
        let seat = instruction_set.get_seat()?;
        let seat_id = seat.id();
        
        if seat_id < min_seat_id {
            min_seat_id = seat_id;
        }

        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }

        all_seat_ids.remove(&seat_id);
    }
    
    let remaining_seat_id = all_seat_ids
        .iter()
        .filter(|id| { **id >= min_seat_id && **id <= max_seat_id })
        .nth(0)
        .expect("unable to disambiguate seat id");
    
    Ok(*remaining_seat_id)
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

