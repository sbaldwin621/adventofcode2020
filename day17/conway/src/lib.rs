use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use cube::Cube;

pub mod config;
mod cube;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    // let filename = config.filename;

    // let lines = read_lines(filename)?;
    // for _line in lines {
    //     // do something with lines
    // }

    // .#.
    // ..#
    // ###
    let mut cube = Cube::from(vec![(1, 0, 0), (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0)]);

    println!("{}", cube);

    for _ in 0..6 {
        cube.step();
        println!("{}", cube);
    }

    Ok(cube.len())
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
