use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use seating::{SeatingChart, SeatingChartRow};

pub mod config;
mod seating;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;

    let mut rows = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines {
        rows.push(line?.parse::<SeatingChartRow>()?);
    }

    let mut chart = SeatingChart::new(rows)?;
    println!("initial:\n{}", chart);

    for step in 1.. {
        let change_count = chart.step();
        println!("step {}, {} changes:\n{}", step, change_count, chart);

        if change_count == 0 {
            break;
        }
    }

    let occupied_count = chart.occupied_count();

    Ok(occupied_count)
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
