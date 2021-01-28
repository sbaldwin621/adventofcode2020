use std::error::Error;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;
use std::thread::current;

use config::Config;
use tiles::{Tile, Tileset};

pub mod config;
mod tiles;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;
    let input = read_to_string(filename)?;

    let mut tiles = Vec::new();
    
    let tile_strings = input.split("\n\n");
    for tile_string in tile_strings {
        let tile = tile_string.parse::<Tile>()?;
        tiles.push(tile);
    }

    let tileset = Tileset::new(tiles);

    let completed_puzzle = tileset.get_completed_puzzle()?;

    let mut current_puzzle = completed_puzzle;
    for _ in 0..4 {
        if current_puzzle.monster_count > 0 {
            break;
        }

        current_puzzle = current_puzzle.rotate();
    }

    println!("{}", current_puzzle);

    let result = current_puzzle.get_roughness_score();

    Ok(result)
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
