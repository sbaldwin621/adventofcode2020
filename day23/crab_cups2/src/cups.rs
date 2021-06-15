use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use petgraph::data::Build;
use petgraph::graph::DiGraph;
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Dfs;

#[derive(Debug)]
pub struct CupSet { 
    cups: DiGraphMap<u32, ()>,
    current_cup: u32,
    smallest_cup: u32,
    largest_cup: u32
}

impl CupSet {
    pub fn new(cups: Vec<u32>) -> CupSet {
        let current_cup = cups[0];
        let smallest_cup = *cups.iter().min().unwrap();
        let largest_cup = *cups.iter().max().unwrap();

        let mut cups_graph: DiGraphMap<u32, ()> = DiGraphMap::new(); 

        let mut previous_cup = None;
        for cup in cups {
            cups_graph.add_node(cup);
            if let Some(previous_cup) = previous_cup {
                cups_graph.add_edge(previous_cup, cup, ());
            }

            previous_cup = Some(cup);
        }

        if let Some(previous_cup) = previous_cup {
            cups_graph.add_edge(previous_cup, current_cup, ());
        }

        CupSet { cups: cups_graph, current_cup, smallest_cup, largest_cup }
    }

    pub fn step(&mut self) {
        let picked_up_cups = self.pick_up_cups();
        let (first, second, third) = picked_up_cups;

        println!("pick up: {}, {}, {}", first, second, third);

        let destination = self.find_destination_cup(picked_up_cups);

        println!("destination: {}", destination);

        let mut dfs = Dfs::new(&self.cups, destination);
        dfs.next(&self.cups);
        
        let next = dfs.next(&self.cups).unwrap();

        self.cups.remove_edge(destination, next);
        self.cups.add_edge(destination, first, ());
        self.cups.add_edge(third, next, ());

        // Traverse from current cup to find next cup
        let mut dfs = Dfs::new(&self.cups, self.current_cup);
        dfs.next(&self.cups);

        self.current_cup = dfs.next(&self.cups).unwrap();

        // println!("pick up {:?}", picked_up_cups);

        // let destination_index = self.find_destination_cup() + 1;

        // for &cup in picked_up_cups.iter().rev() {
        //     self.cups.insert(destination_index, cup);
        // }

        // let new_current_cup_index = (self.find_current_cup() + 1) % self.cups.len();
        // self.current_cup = self.cups[new_current_cup_index];
    }

    pub fn answer(&self) -> String {
        let mut dfs = Dfs::new(&self.cups, 1);
        dfs.next(&self.cups);


        let mut answer = String::with_capacity(self.cups.node_count() - 1);

        while let Some(cup) = dfs.next(&self.cups) {
            if cup == 1 {
                break;
            }

            answer.push_str(&cup.to_string());
        }

        answer

        // let count = self.cups.len();
        // let mut answer = String::with_capacity(count - 1);
        
        // let one = self.find_cup(1) + 1;
        // for i in one..one+count-1 {
        //     let cup = self.cups[i % count];
        //     answer.push_str(&cup.to_string());
        // }

        // answer
    }

    fn find_cup(&self, value: u32) -> usize {
        todo!()
        
        // self.cups.iter().position(|&i| i == value).unwrap()
    }

    fn find_current_cup(&self) -> usize {
        todo!()

        // self.find_cup(self.current_cup)
    }

    fn find_destination_cup(&self, picked_up_cups: (u32, u32, u32)) -> u32 {
        let (first, second, third) = picked_up_cups;
        let mut target = self.current_cup - 1;
        loop {
            if target < self.smallest_cup {
                target = self.largest_cup;
            } else if target != first && target != second && target != third {
                return target;
            } else {
                target = target - 1;
            }
        }
    }

    fn pick_up_cups(&mut self) -> (u32, u32, u32) {
        let current_cup = self.current_cup;
        
        let mut dfs = Dfs::new(&self.cups, current_cup);
        dfs.next(&self.cups).unwrap();

        let first = dfs.next(&self.cups).unwrap();
        let second = dfs.next(&self.cups).unwrap();
        let third = dfs.next(&self.cups).unwrap();
        
        let next = dfs.next(&self.cups).unwrap();

        // Disconnect the three cups from the graph
        self.cups.remove_edge(current_cup, first);
        self.cups.remove_edge(third, next);
        self.cups.add_edge(current_cup, next, ());

        (first, second, third)

        // let mut picked_up_cups = Vec::new();

        // for _ in 0..3 {
        //     let current_cup_index = self.find_current_cup();
        //     let next_cup_index = (current_cup_index + 1) % self.cups.len();
        //     let cup = self.cups.remove(next_cup_index);
        //     picked_up_cups.push(cup);
        // }
        
        // picked_up_cups
    }
}

impl FromStr for CupSet {
    type Err = ParseCupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cups = Vec::new();
        
        for c in s.chars() {
            let digit = c.to_digit(10).ok_or(ParseCupError::InvalidDigit)?;
            cups.push(digit);
        }
        
        Ok(CupSet::new(cups))
    }
}

impl Display for CupSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        s.push_str(&self.current_cup.to_string());

        let mut dfs = Dfs::new(&self.cups, self.current_cup);
        dfs.next(&self.cups);

        while let Some(cup) = dfs.next(&self.cups) {
            if cup == self.current_cup {
                break;
            }

            s.push_str(&cup.to_string());
        }

        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ParseCupError {
    InvalidDigit
}

impl Display for ParseCupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseCupError::InvalidDigit => "Invalid digit",
        })
    }
}

impl Error for ParseCupError { }
