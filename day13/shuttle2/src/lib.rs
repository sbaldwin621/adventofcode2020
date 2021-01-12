use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use schedule::Schedule;

pub mod config;
mod schedule;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;

    let lines = read_lines(filename)?;
    if lines.len() != 2 {
        return Err(ApplicationError::WrongLineCount.into());
    }

    // Day 2, ignore line 1
    // let timestamp = lines[0].parse::<usize>()
    //     .map_err(|_| ApplicationError::InvalidTimestamp)?;

    let schedule = lines[1].parse::<Schedule>()?;

    let earliest_time = schedule.find_earliest_time();

    Ok(earliest_time)
}

fn find_best_bus(timestamp: usize, schedule: Vec<usize>) -> Option<BusScore> {
    let mut best_score: Option<BusScore> = None;

    for bus in schedule {
        let d = timestamp / bus;
        let remainder = timestamp - d * bus;

        if remainder == 0 {
            return Some(BusScore { id: bus, minutes_to_wait: 0 });
        }

        let minutes_to_wait = bus - remainder;

        if let Some(existing_score) = &best_score {
            if minutes_to_wait < existing_score.minutes_to_wait {
                best_score = Some(BusScore { id: bus, minutes_to_wait });
            }
        } else {
            best_score = Some(BusScore { id: bus, minutes_to_wait });
        }
        
        println!("{} / {} = {}; rem {}", timestamp, bus, d, remainder);
    }

    best_score
}

#[derive(Debug)]
struct BusScore {
    id: usize,
    minutes_to_wait: usize
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let mut result = Vec::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        result.push(line?);
    }

    Ok(result)
}

#[derive(Debug)]
pub enum ApplicationError {
    WrongLineCount,
    InvalidTimestamp,
    InvalidSchedule,
    BestScoreNotFound
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ApplicationError::WrongLineCount => "input was not exactly 2 lines",
            ApplicationError::InvalidTimestamp => "timestamp was invalid",
            ApplicationError::InvalidSchedule => "schedule was invalid",
            ApplicationError::BestScoreNotFound => "best score could not be found"
        })
    }
}

impl Error for ApplicationError { }
