use std::{collections::HashSet, error::Error, fmt::Display, str::FromStr};

pub struct Form {
    entries: Vec<FormEntry>
}

impl Form {
    pub fn new() -> Form {
        let entries = Vec::new();
        Form { entries }
    }
    
    pub fn insert(&mut self, entry: FormEntry) {
        self.entries.push(entry);
    }

    pub fn count(&self) -> usize {
        if self.entries.len() == 0 {
            return 0;
        }

        let mut compiled_answers = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .collect::<HashSet<_>>();

        for entry in self.entries.iter() {
            compiled_answers = compiled_answers.intersection(&entry.answers).cloned().collect();
        }

        compiled_answers.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[derive(Debug)]
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