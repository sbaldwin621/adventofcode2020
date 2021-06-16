use std::collections::HashMap;

use crate::instructions::{Direction, InstructionSet};

#[derive(Debug)]
pub enum Tile {
    White,
    Black
}

impl Tile {
    pub fn flip(&self) -> Tile {
        match self {
            Tile::White => Tile::Black,
            Tile::Black => Tile::White,
        }
    }
}

#[derive(Debug)]
pub struct Lobby {
    tiles: HashMap<(i32, i32, i32), Tile>,
    black_count: usize
}

impl Lobby {
    pub fn new() -> Lobby {
        let tiles = HashMap::new();
        Lobby { tiles, black_count: 0 }
    }

    pub fn follow_instructions(&mut self, instruction_set: &InstructionSet) {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        for direction in instruction_set.iter() {
            let (x_delta, y_delta, z_delta) = Lobby::direction_to_delta(direction);

            x += x_delta;
            y += y_delta;
            z += z_delta;
        }

        let coordinates = (x, y, z);
        let tile = self.tiles.entry(coordinates)
            .or_insert(Tile::White);

        *tile = tile.flip();

        if let Tile::Black = tile {
            self.black_count += 1;
        } else {
            self.black_count -= 1;
        }
    }

    fn direction_to_delta(direction: &Direction) -> (i32, i32, i32) {
        match direction {
            Direction::West => (-1, 1, 0),
            Direction::Northwest => (0, 1, -1),
            Direction::Northeast => (1, 0, -1),
            Direction::East => (1, -1, 0),
            Direction::Southeast => (0, -1, 1),
            Direction::Southwest => (-1, 0, 1),
        }
    }

    pub fn count(&self) -> usize {
        self.black_count
    }
}