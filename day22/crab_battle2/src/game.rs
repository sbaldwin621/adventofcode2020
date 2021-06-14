use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use crate::parser::puzzle_input;

#[derive(Debug)]
pub enum GameStatus {
    Continuing,
    Player1,
    Player2
}

#[derive(Debug)]
pub enum GameResult {
    Player1(usize),
    Player2(usize)
}

#[derive(Debug)]
pub struct PuzzleInput {
    player1: Deck,
    player2: Deck,
    previous_rounds: HashSet<usize>
}

impl PuzzleInput {
    pub fn new(player1: Deck, player2: Deck) -> PuzzleInput {
        let previous_rounds = HashSet::new();
        PuzzleInput { player1, player2, previous_rounds }
    }

    pub fn play(&mut self) -> GameResult {
        loop {
            let step = self.step();
            match step {
                GameStatus::Continuing => { }
                GameStatus::Player1 => return GameResult::Player1(self.player1.score()),
                GameStatus::Player2 => return GameResult::Player2(self.player2.score())
            }
        }
    }

    pub fn step(&mut self) -> GameStatus {
        let hash1 = self.player1.hash();
        let hash2 = self.player2.hash();
        let game_hash = hash1.wrapping_mul(13) ^ hash2;

        if self.previous_rounds.contains(&game_hash) {
            return GameStatus::Player1;
        }

        self.previous_rounds.insert(game_hash);

        let card1 = self.player1.flip_card();
        let card2 = self.player2.flip_card();

        let step_result: GameResult;
        if card1 <= self.player1.len() && card2 <= self.player2.len() {
            let new_player1 = self.player1.clone(card1);
            let new_player2 = self.player2.clone(card2);
            let mut recursive_game = PuzzleInput::new(new_player1, new_player2);
            step_result = recursive_game.play();
        } else if card1 > card2 {
            step_result = GameResult::Player1(0);
        } else {
            step_result = GameResult::Player2(0);
        }

        match step_result {
            GameResult::Player1(_) => {
                self.player1.add_card(card1);
                self.player1.add_card(card2);

                if self.player2.is_empty() {
                    GameStatus::Player1
                } else {
                    GameStatus::Continuing
                }
            },
            GameResult::Player2(_) => {
                self.player2.add_card(card2);
                self.player2.add_card(card1);

                if self.player1.is_empty() {
                    GameStatus::Player2
                } else {
                    GameStatus::Continuing
                }
            }
        }
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

    pub fn len(&self) -> usize {
        self.cards.len()
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

    pub fn clone(&self, len: usize) -> Deck {
        let mut new_cards = VecDeque::new();
        for card in self.cards.iter().take(len) {
            new_cards.push_back(*card);
        }

        Deck { cards: new_cards }
    }

    pub fn hash(&self) -> usize {
        let mut hash: usize = 0;

        for card in self.cards.iter() {
            hash = hash.wrapping_mul(13) ^ card;
        }

        hash
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