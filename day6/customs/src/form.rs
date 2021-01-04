use std::{collections::HashSet, error::Error, fmt::Display, str::FromStr};

pub struct Form {
    answers: HashSet<char>
}

impl Form {
    pub fn new() -> Form {
        let answers = HashSet::new();
        Form { answers }
    }
    
    pub fn len(&self) -> usize {
        self.answers.len()
    }

    pub fn extend(&mut self, entry: &FormEntry) {
        self.answers.extend(&entry.answers);
    }

    pub fn clear(&mut self) {
        self.answers.clear();
    }
}

pub struct FormEntry {
    answers: HashSet<char>
}

impl FormEntry {
    pub fn len(&self) -> usize {
        self.answers.len()
    }
}

impl FromStr for FormEntry {
    type Err = ParseFormEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut answers = HashSet::new();

        for char in s.chars() {
            answers.insert(char);
        }

        Ok(FormEntry { answers })
    }
}

#[derive(Debug)]
pub struct ParseFormEntryError { }

impl Error for ParseFormEntryError { }

impl Display for ParseFormEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}