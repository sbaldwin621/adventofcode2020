use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;

    let mut adapters = Vec::new();

    adapters.push(0); // outlet with effective joltage of 0

    let lines = read_lines(filename)?;
    for line in lines {
        let adapter = line?.parse::<usize>()?;
        adapters.push(adapter);
    }

    adapters.sort();

    adapters.push(adapters.last().unwrap() + 3); // device with joltage of 3 higher than highest adapter

    let mut edges = HashMap::new();
    
    for i in 0..adapters.len() {
        let adapter = adapters[i];
        let mut edges_for_adapter = Vec::new();

        for look_forward in i+1..=i+3 {
            if let Some(forward) = adapters.get(look_forward) {
                if forward - adapter <= 3 {
                    edges_for_adapter.push(*forward);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if edges_for_adapter.len() > 0 {
            edges.insert(adapter, edges_for_adapter);
        }
    }

    println!("{:?}", count_paths(0, &edges));

    Ok(0)
}

fn count_paths(adapter: usize, edges: &HashMap<usize, Vec<usize>>) -> usize {
    let mut memo = HashMap::new();

    count_paths_helper(adapter, edges, &mut memo)
}

fn count_paths_helper(adapter: usize, edges: &HashMap<usize, Vec<usize>>, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(memoized_answer) = memo.get(&adapter) {
        return *memoized_answer;
    }

    if let Some(edges_for_adapter) = edges.get(&adapter) {
        let mut accum = 0;
        for edge in edges_for_adapter {
            accum = accum + count_paths_helper(*edge, edges, memo)
        }

        memo.insert(adapter, accum);

        accum
    } else {
        memo.insert(adapter, 1);

        1
    }
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
