use std::error::Error;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use food::FoodList;
use parser::food_list;

pub mod config;
mod food;
mod parser;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;

    let input = read_to_string(filename)?;

    let (_, foods) = food_list(&input).map_err(|e| ApplicationError::ParseInputError(e.to_string()))?;

    println!("{:?}", foods);

    Ok(0)
}

fn count_nonallergens(food_list: &FoodList) -> usize {
    let mut count = 0;

    

    count
}

#[derive(Debug)]
pub enum ApplicationError {
    ParseInputError(String)
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ApplicationError::ParseInputError(error) => error
        })
    }
}

impl Error for ApplicationError { }
