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
    tiles: HashMap<(i32, i32, i32), Tile>
}

impl Lobby {
    pub fn new() -> Lobby {
        let tiles = HashMap::new();
        Lobby { tiles }
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
    }

    pub fn pass_day(&mut self) {
        let mut neighbors = HashMap::new();

        for (coordinate, tile) in self.tiles.iter() {
            if let Tile::Black = tile {
                for neighbor in Lobby::get_coordinate_neighbors(*coordinate) {
                    let neighbor_count = neighbors.entry(neighbor).or_insert(0);
                    *neighbor_count += 1; 
                }
            }
        }

        let mut new_tiles = HashMap::new();
        for (coordinate, neighbor_count) in neighbors {
            let will_be_black = match self.tiles.get(&coordinate).unwrap_or(&Tile::White) {
                Tile::White => {
                    neighbor_count == 2
                },
                Tile::Black => {
                    neighbor_count == 1 || neighbor_count == 2
                }
            };

            if will_be_black {
                new_tiles.insert(coordinate, Tile::Black);               
            }
        }

        self.tiles = new_tiles;
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

    fn get_coordinate_neighbors((x, y, z): (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
        vec![
            (x - 1, y + 1, z),
            (x, y + 1, z - 1),
            (x + 1, y, z - 1),
            (x + 1, y - 1, z),
            (x, y - 1, z + 1),
            (x - 1, y, z + 1)
        ]
    }

    pub fn count(&self) -> usize {
        self.tiles.iter().filter(|(_, tile)| matches!(tile, Tile::Black)).count()
    }
}