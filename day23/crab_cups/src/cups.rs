use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct CupSet { 
    cups: Vec<u32>,
    current_cup: u32,
    smallest_cup: u32,
    largest_cup: u32
}

impl CupSet {
    pub fn new(cups: Vec<u32>) -> CupSet {
        let current_cup = cups[0];
        let smallest_cup = *cups.iter().min().unwrap();
        let largest_cup = *cups.iter().max().unwrap();

        CupSet { cups, current_cup, smallest_cup, largest_cup }
    }

    pub fn step(&mut self) {
        let picked_up_cups = self.pick_up_cups();
        println!("pick up {:?}", picked_up_cups);

        let destination_index = self.find_destination_cup() + 1;

        for &cup in picked_up_cups.iter().rev() {
            self.cups.insert(destination_index, cup);
        }

        let new_current_cup_index = (self.find_current_cup() + 1) % self.cups.len();
        self.current_cup = self.cups[new_current_cup_index];
    }

    pub fn answer(&self) -> String {
        let count = self.cups.len();
        let mut answer = String::with_capacity(count - 1);
        
        let one = self.find_cup(1) + 1;
        for i in one..one+count-1 {
            let cup = self.cups[i % count];
            answer.push_str(&cup.to_string());
        }

        answer
    }

    fn find_cup(&self, value: u32) -> usize {
        self.cups.iter().position(|&i| i == value).unwrap()
    }

    fn find_current_cup(&self) -> usize {
        self.find_cup(self.current_cup)
    }

    fn find_destination_cup(&self) -> usize {
        let mut target = self.current_cup - 1;
        loop {
            if let Some(destination) = self.cups.iter().position(|&i| i == target) {
                return destination;
            }

            if target == self.current_cup {
                panic!("looped cups");
            } else if target <= self.smallest_cup {
                target = self.largest_cup;
            } else {
                target = target - 1;
            }
        }
    }

    fn pick_up_cups(&mut self) -> Vec<u32> {
        let mut picked_up_cups = Vec::new();

        for _ in 0..3 {
            let current_cup_index = self.find_current_cup();
            let next_cup_index = (current_cup_index + 1) % self.cups.len();
            let cup = self.cups.remove(next_cup_index);
            picked_up_cups.push(cup);
        }
        
        picked_up_cups
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
