use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

mod bag;
mod rules;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;

    let lines = read_lines(filename)?;
    for line in lines {
        // do something with linesp
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

