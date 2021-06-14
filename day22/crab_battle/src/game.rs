use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use crate::parser::puzzle_input;

#[derive(Debug)]
pub enum GameStatus {
    Continuing,
    WinnerFound(usize)
}

#[derive(Debug)]
pub struct PuzzleInput {
    player1: Deck,
    player2: Deck
}

impl PuzzleInput {
    pub fn new(player1: Deck, player2: Deck) -> PuzzleInput {
        PuzzleInput { player1, player2 }
    }

    pub fn step(&mut self) -> GameStatus {
        let card1 = self.player1.flip_card();
        let card2 = self.player2.flip_card();

        if card1 > card2 {
            self.player1.add_card(card1);
            self.player1.add_card(card2);

            if self.player2.is_empty() {
                let winning_score = self.player1.score();
                return GameStatus::WinnerFound(winning_score);
            }
        } else {
            self.player2.add_card(card2);
            self.player2.add_card(card1);

            if self.player1.is_empty() {
                let winning_score = self.player2.score();
                return GameStatus::WinnerFound(winning_score);
            }
        }

        GameStatus::Continuing
    }
}

impl FromStr for PuzzleInput {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        puzzle_input(s)
            .map(|(_, result)| result)
            .map_err(|_| "failed to parse puzzle input")
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: VecDeque<usize>
}

impl Deck {
    pub fn new(cards: Vec<usize>) -> Deck {
        let cards = VecDeque::from(cards);
        Deck { cards }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.len() == 0
    }

    pub fn score(&self) -> usize {
        let mut score = 0;

        let length = self.cards.len();
        for (i, card) in self.cards.iter().enumerate() {
            score += (length - i) * card;
        }

        score
    }

    pub fn flip_card(&mut self) -> usize {
        self.cards.pop_front().unwrap()
    }

    pub fn add_card(&mut self, card: usize) {
        self.cards.push_back(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score() {
        let deck = Deck::new(vec!(3, 2, 10, 6, 8, 5, 9, 4, 7, 1));

        assert_eq!(306, deck.score())
    }
}