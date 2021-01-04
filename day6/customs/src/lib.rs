use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

mod form;
use form::{Form, FormEntry};

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;

    let mut sum = 0;
    let mut form = Form::new();

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        if line.len() > 0 {
            let form_entry = line.parse::<FormEntry>()?;
            form.extend(&form_entry);
        } else {
            sum += form.len();
            form.clear();
        }
    }

    // Add final form
    sum += form.len();

    Ok(sum)
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

