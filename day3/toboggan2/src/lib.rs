use std::{error::Error, fmt::Display, str::FromStr};
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;
    let mut runners = config.slopes.iter()
        .map(|slope| TobogganRunner::new(slope))
        .collect::<Vec<TobogganRunner>>();

    let lines = read_lines(filename)?;
    for (y, line) in lines.enumerate() {
        let latitude = line?.parse::<Latitude>()?;

        for runner in runners.iter_mut() {
            runner.run_latitude(&latitude, y)
        }
    }

    let count = runners.iter()
        .fold(1, |accum, runner| accum * runner.count);

    Ok(count)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct TobogganRunner {
    slope: (usize, usize),

    x: usize,
    count: usize
}

impl TobogganRunner {
    pub fn new(slope: &(usize, usize)) -> TobogganRunner {
        TobogganRunner { slope: slope.clone(), x: 0, count: 0 }
    }

    pub fn run_latitude(&mut self, latitude: &Latitude, y: usize) {
        let (horizontal_speed, vertical_speed) = self.slope;

        // Skip this latitude if shouldn't be visited based on vertical speed
        if y % vertical_speed != 0 {
            return;
        }

        let i = self.x % latitude.trees.len();
        let tree_at_i = latitude.trees[i];
        if tree_at_i {
            self.count = self.count + 1;
        }

        self.x = self.x + horizontal_speed;
    }
}

#[derive(Debug)]
struct Latitude {
    trees: Vec<bool>
}

impl FromStr for Latitude {
    type Err = ParseLatitudeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ..##.......
        let trees: Vec<bool> = s.chars().map(|c| { 
            match c {
                '#' => true,
                _ => false
            }
        }).collect();

        Ok(Latitude { trees })
    }
}

#[derive(Debug)]
struct ParseLatitudeError { }

impl Display for ParseLatitudeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "couldn't parse latitude")
    }
}

impl Error for ParseLatitudeError { }

pub struct Config {
    pub filename: String,
    pub slopes: Vec<(usize, usize)>
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let len = args.len();

        if len < 3 {
            return Err(Box::new(ParseConfigError { kind: ParseConfigErrorKind::NotEnoughArguments }));
        }

        if len % 2 != 0 {
            return Err(Box::new(ParseConfigError { kind: ParseConfigErrorKind::MismatchedSlopes }))
        }

        let filename = args[1].clone();
        let mut slopes = Vec::new();
        
        for i in (2..len).step_by(2) {
            let x = args[i].parse::<usize>()?;
            let y = args[i + 1].parse::<usize>()?;

            slopes.push((x, y));
        }
    
        Ok(Config { filename, slopes })
    }
}

#[derive(Debug)]
struct ParseConfigError {
    kind: ParseConfigErrorKind
}

impl Display for ParseConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match &self.kind {
            ParseConfigErrorKind::NotEnoughArguments => "not enough arguments",
            ParseConfigErrorKind::MismatchedSlopes => "slopes must be given in pairs"
        })
    }
}

impl Error for ParseConfigError { }

#[derive(Debug)]
enum ParseConfigErrorKind {
    NotEnoughArguments,
    MismatchedSlopes
}