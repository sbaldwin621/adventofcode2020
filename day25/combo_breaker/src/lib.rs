use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;

use crate::crypto::{derive_encryption_key, derive_loop_size};

pub mod config;
pub mod crypto;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;

    let lines = read_lines(filename)?;
    
    let mut iter = lines.take(2);
    let card_public_key = iter.next().unwrap().unwrap().parse::<u64>()?;
    let door_public_key = iter.next().unwrap().unwrap().parse::<u64>()?;

    println!("{:?} {:?}", card_public_key, door_public_key);

    let card_loop_size = derive_loop_size(7, card_public_key);
    let door_loop_size = derive_loop_size(7, door_public_key);

    println!("{:?} {:?}", card_loop_size, door_loop_size);

    let encryption_key = derive_encryption_key(card_public_key, door_loop_size);

    println!("{:?}", encryption_key);

    Ok(0)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub enum ApplicationError {
    AnError
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            AnError => "an error occurred"  
        })
    }
}

impl Error for ApplicationError { }
