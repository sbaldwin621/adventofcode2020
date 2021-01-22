use std::error::Error;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;
use std::thread::current;

use config::Config;

pub mod config;
mod tiles;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;
    let input = read_to_string(filename)?;

    let tile_strings = input.split("\n\n");
    for tile_string in tile_strings {
        
    }

    Ok(0)
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
