use std::error::Error;
use std::fmt::Display;

pub struct Config {
    pub filename: String,
    pub turns: usize
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ParseConfigError> {
        if args.len() < 3 {
            return Err(ParseConfigError::NotEnoughArguments);
        }

        let filename = args[1].clone();
        let turns = args[2].parse::<usize>()
            .map_err(|_| ParseConfigError::InvalidTurnCount)?;
        
        Ok(Config { filename, turns })
    }
}

#[derive(Debug)]
pub enum ParseConfigError {
    NotEnoughArguments,
    InvalidTurnCount
}

impl Display for ParseConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseConfigError::NotEnoughArguments => "not enough arguments",
            ParseConfigError::InvalidTurnCount => "turn count not a valid number"
        })
    }
}

impl Error for ParseConfigError { }