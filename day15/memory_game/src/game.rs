use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

pub struct NumberGame {
    starting_numbers: Vec<usize>
}

impl NumberGame {
    pub fn new(starting_numbers: Vec<usize>) -> NumberGame {
        NumberGame { starting_numbers }
    }

    pub fn iter(&self) -> NumberGameIter {
        NumberGameIter::new(&self.starting_numbers)
    }
}

impl FromStr for NumberGame {
    type Err = ParseNumberGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut starting_numbers = Vec::new();

        starting_numbers.iter();

        for number in s.split(',') {
            let parsed_number = number.parse::<usize>()
                .map_err(|_| ParseNumberGameError::UhOh)?;

            starting_numbers.push(parsed_number);
        }

        Ok(NumberGame { starting_numbers })
    }
}

#[derive(Debug)]
pub enum ParseNumberGameError {
    UhOh
}

impl Display for ParseNumberGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseNumberGameError::UhOh => "uh oh"
        })
    }
}

impl Error for ParseNumberGameError { }

pub struct NumberGameIter<'a> {
    starting_numbers: &'a Vec<usize>,
    turn: usize,
    last_spoken_number: Option<usize>,
    state: HashMap<usize, usize>
}

impl NumberGameIter<'_> {
    pub fn new(starting_numbers: &Vec<usize>) -> NumberGameIter {
        NumberGameIter { starting_numbers, turn: 0, last_spoken_number: None, state: HashMap::new() }
    }
}

impl Iterator for NumberGameIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let number = if self.turn < self.starting_numbers.len() {
            self.starting_numbers[self.turn]
        } else if let Some(last_spoken_number) = self.last_spoken_number {
            if let Some(turn_last_spoken) = self.state.get(&last_spoken_number) {
                self.turn - turn_last_spoken - 1
            } else {
                0
            }
        } else { 
            panic!("invalid internal state")
        };

        if let Some(last_spoken_number) = self.last_spoken_number {
            self.state.insert(last_spoken_number, self.turn - 1);
        }

        self.last_spoken_number = Some(number);
        self.turn = self.turn + 1;

        Some(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let starting_numbers = vec![0, 3, 6];
        let mut stream = NumberGameIter::new(&starting_numbers);
        let result = stream.nth(2019).unwrap();

        assert_eq!(436, result);
    }

    #[test]
    fn example2() {
        let starting_numbers = vec![1, 3, 2];
        let mut stream = NumberGameIter::new(&starting_numbers);
        let result = stream.nth(2019).unwrap();

        assert_eq!(1, result);
    }

    #[test]
    fn example3() {
        let starting_numbers = &vec![1, 2, 3];
        let mut stream = NumberGameIter::new(&starting_numbers);
        let result = stream.nth(2019).unwrap();

        assert_eq!(27, result);
    }
}