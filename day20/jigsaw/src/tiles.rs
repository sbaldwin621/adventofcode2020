use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

const LEFT_EDGE_MASK: u32 = 0b1000000000;
const RIGHT_EDGE_MASK: u32 = 0b0000000001;

#[derive(Debug)]
pub struct Tileset {
    pub tiles: HashMap<u32, Tile>,
    pub size: usize,
    edge_map: HashMap<u32, Vec<u32>>
}

impl Tileset {
    pub fn new(tiles: Vec<Tile>) -> Tileset {
        let size = (tiles.len() as f64).sqrt() as usize;
        let mut tile_map = HashMap::new();

        for tile in tiles {
            tile_map.insert(tile.id, tile);
        }

        let edge_map = Tileset::get_edge_map(&tile_map);
        
        Tileset { tiles: tile_map, size, edge_map }
    }

    pub fn get_completed_puzzle(&self) -> Result<Vec<&TileVariation>, PuzzleError> {
        let mut result = Vec::new();

        let mut remaining_pieces = HashSet::new();
        for (id, _) in self.tiles.iter() {
            remaining_pieces.insert(*id);
        }

        let topleft = self.find_top_left_corner()
            .ok_or(PuzzleError::CouldntFindTopLeftCorner)?;
        
        remaining_pieces.remove(&topleft.id);
        result.push(topleft);

        // Top edge
        for x in 1..self.size {
            let piece_to_left = *result.last().unwrap();
            let edge_to_match = piece_to_left.right_edge;

            let matching_variation = remaining_pieces.iter()
                .map(|id| self.tiles.get(id).unwrap())
                .flat_map(|tile| tile.variations.iter())
                .find(|variation| {
                    !self.is_edge_shared(variation.top_edge) &&
                    variation.left_edge == edge_to_match
                })
                .ok_or(PuzzleError::CouldFindMatch)?;
            
            remaining_pieces.remove(&matching_variation.id);

            result.push(matching_variation);
        }

        for y in 1..self.size {
            for x in 0..self.size {
                let piece_to_left = if x > 0 {
                    result.get((y * self.size + x - 1) as usize)
                } else {
                    None
                };
                let piece_above = *result.get(((y - 1) * self.size + x) as usize).unwrap();

                let matching_variation = remaining_pieces.iter()
                    .map(|id| self.tiles.get(id).unwrap())
                    .flat_map(|tile| tile.variations.iter())
                    .find(|variation| {
                        if let Some(piece_to_left) = piece_to_left {
                            variation.left_edge == piece_to_left.right_edge &&
                            variation.top_edge == piece_above.bottom_edge
                        } else {
                            !self.is_edge_shared(variation.left_edge) &&
                            variation.top_edge == piece_above.bottom_edge
                        }
                    })
                    .unwrap();

                remaining_pieces.remove(&matching_variation.id);
    
                result.push(matching_variation);
            }
        }

        Ok(result)
    }
    
    fn get_edge_map(tiles: &HashMap<u32, Tile>) -> HashMap<u32, Vec<u32>> {
        let mut edge_map: HashMap<u32, Vec<u32>> = HashMap::new();

        for (tile_id, tile) in tiles.iter() {
            for edge in tile.unique_edges.iter() {
                if let Some(existing_vec) = edge_map.get_mut(edge) {
                    existing_vec.push(*tile_id);
                } else {
                    edge_map.insert(*edge, vec![*tile_id]);
                }
            }
        }

        edge_map
    }

    fn find_top_left_corner(&self) -> Option<&TileVariation> {
        for (_, tile) in self.tiles.iter() {
            for variation in tile.variations.iter() {
                if !self.is_edge_shared(variation.left_edge) &&
                   !self.is_edge_shared(variation.top_edge) {
                    return Some(variation);
                }
            }
        }

        None
    }

    fn is_edge_shared(&self, edge: u32) -> bool {
        if let Some(tiles) = self.edge_map.get(&edge) {
            if tiles.len() == 1 {
                return false;
            } else {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
pub enum PuzzleError {
    CouldntFindTopLeftCorner,
    CouldFindMatch
}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            PuzzleError::CouldntFindTopLeftCorner => "couldn't find top-left corner",
            PuzzleError::CouldFindMatch => "couldn't find match"
        })
    }
}

impl Error for PuzzleError { }

#[derive(Debug)]
pub struct Tile {
    pub id: u32,
    pub variations: Vec<TileVariation>,
    pub unique_edges: HashSet<u32>
}

impl Tile {
    pub fn new(id: u32, rows: Vec<u32>) -> Tile {
        let variations = Tile::calculate_variations(id, &rows);
        let unique_edges = Tile::get_unique_edges(&variations);
        Tile { id, variations, unique_edges }
    }

    fn calculate_variations(id: u32, rows: &Vec<u32>) -> Vec<TileVariation> {

        let mut variations = Vec::new();

        // Normal
        let original = TileVariation::new(id, rows.clone());
        let rotated = original.rotate();
        let rotated2 = rotated.rotate();
        let rotated3 = rotated2.rotate();
        let flipped = original.flip();
        let flipped_rotated = flipped.rotate();
        let flipped_rotated2 = flipped_rotated.rotate();
        let flipped_rotated3 = flipped_rotated2.rotate();

        variations.push(original);
        variations.push(rotated);
        variations.push(rotated2);
        variations.push(rotated3);
        variations.push(flipped);
        variations.push(flipped_rotated);
        variations.push(flipped_rotated2);
        variations.push(flipped_rotated3);

        variations
    }

    fn get_unique_edges(variations: &Vec<TileVariation>) -> HashSet<u32> {
        let mut edges = HashSet::new();

        for variation in variations.iter() {
            edges.insert(variation.top_edge);
            edges.insert(variation.right_edge);
            edges.insert(variation.bottom_edge);
            edges.insert(variation.left_edge);
        }
        
        edges
    }
}

#[derive(Debug)]
pub struct TileVariation {
    pub id: u32,
    pub rows: Vec<u32>,
    pub top_edge: u32,
    pub right_edge: u32,
    pub bottom_edge: u32,
    pub left_edge: u32
}

impl TileVariation {
    pub fn new(id: u32, rows: Vec<u32>) -> TileVariation {
        let (top_edge, right_edge, bottom_edge, left_edge) = TileVariation::calculate_edges(&rows);

        TileVariation { id, rows, top_edge, right_edge, bottom_edge, left_edge }
    }

    pub fn flip(&self) -> TileVariation {
        let rows = self.rows
            .iter()
            .map(|row| TileVariation::flip_row(*row))
            .collect::<Vec<_>>();

        TileVariation::new(self.id, rows)
    }

    pub fn rotate(&self) -> TileVariation {
        let mut new_rows = Vec::new();

        if self.rows.len() != 10 {
            panic!("incorrect puzzle size");
        }

        let row_0 = self.rows.get(0).unwrap();
        let row_1 = self.rows.get(1).unwrap();
        let row_2 = self.rows.get(2).unwrap();
        let row_3 = self.rows.get(3).unwrap();
        let row_4 = self.rows.get(4).unwrap();
        let row_5 = self.rows.get(5).unwrap();
        let row_6 = self.rows.get(6).unwrap();
        let row_7 = self.rows.get(7).unwrap();
        let row_8 = self.rows.get(8).unwrap();
        let row_9 = self.rows.get(9).unwrap();

        for n in 0..10 {
            let column_mask = 1 << (9 - n);
            let new_row =
                (row_0 & column_mask) >> (9 - n)      |
                (row_1 & column_mask) >> (9 - n) << 1 | 
                (row_2 & column_mask) >> (9 - n) << 2 | 
                (row_3 & column_mask) >> (9 - n) << 3 | 
                (row_4 & column_mask) >> (9 - n) << 4 | 
                (row_5 & column_mask) >> (9 - n) << 5 | 
                (row_6 & column_mask) >> (9 - n) << 6 | 
                (row_7 & column_mask) >> (9 - n) << 7 | 
                (row_8 & column_mask) >> (9 - n) << 8 |
                (row_9 & column_mask) >> (9 - n) << 9;
                
            new_rows.push(new_row);
        }

        let result = TileVariation::new(self.id, new_rows);
        
        // println!("{}", self);
        // println!("{}", result);

        return result;
    }
    
    pub fn edges(&self) -> [u32; 4] {
        [self.top_edge, self.right_edge, self.bottom_edge, self.left_edge]
    }
    
    fn calculate_edges(rows: &Vec<u32>) -> (u32, u32, u32, u32) {
        let first_row = *rows.get(0).unwrap();
        let last_row = *rows.get(rows.len() - 1).unwrap();

        let mut left_edge = 0u32;
        let mut right_edge = 0u32;

        for row in rows.iter() {
            let row = *row;

            left_edge = (left_edge << 1) | ((row & LEFT_EDGE_MASK) >> 9);
            right_edge = (right_edge << 1) | ((row & RIGHT_EDGE_MASK));
        }

        (first_row, right_edge, last_row, left_edge)
    }

    fn flip_row(row: u32) -> u32 {
        row.reverse_bits() >> 22
    }
}

impl Display for TileVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for row in self.rows.iter() {
            result.push_str(&format!("{:010b}\n", row).replace("0", ".").replace("1", "#"));
        }

        writeln!(f, "{}", result)
    }
}

pub struct CompletedPuzzle<'a> {
    pub tiles: Vec<&'a TileVariation>,
    pub size: usize
}

impl<'a> CompletedPuzzle<'a> {
    pub fn new(tiles: Vec<&'a TileVariation>, size: usize) -> CompletedPuzzle<'a> {
        CompletedPuzzle { tiles, size }
    }
}

impl FromStr for Tile {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n").collect::<Vec<_>>();
        let first_line = lines.remove(0);

        if !first_line.starts_with("Tile ") {
            return Err(ParseTileError::IncorrectHeader);
        }

        let tile_id = &first_line[5..9];
        let tile_id = tile_id.parse::<u32>()
            .map_err(|_| ParseTileError::IncorrectHeader)?;

        let mut rows = Vec::new();

        for line in lines {
            let line = line.replace("#", "1").replace(".", "0");
            let row = u32::from_str_radix(&line, 2)
                .map_err(|_| ParseTileError::InvalidRow)?;

            rows.push(row);            
        }

        Ok(Tile::new(tile_id, rows))
    }
}

#[derive(Debug)]
pub enum ParseTileError {
    NotEnoughLines,
    IncorrectHeader,
    InvalidRow
}

impl Display for ParseTileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseTileError::NotEnoughLines => "not enough lines",
            ParseTileError::IncorrectHeader => "incorrect header",
            ParseTileError::InvalidRow => "invalid row"
        })
    }
}

impl Error for ParseTileError { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
//         let tile: Tile = "Tile 2311:
// ..##.#..#.
// ##..#.....
// #...##..#.
// ####.#...#
// ##.##.###.
// ##...#.###
// .#.#.#..##
// ..#....#..
// ###...#.#.
// ..###..###".parse::<Tile>().unwrap();

//         println!("{:?}", tile);

//         assert_eq!(2311, tile.id);
//         assert_eq!(vec![
//             0b0011010010,
//             0b0011100111,
//             0b0111110010,
//             0b0001011001
//         ], tile.edges);
    }

    #[test]
    fn flip_row() {
        let row = 0b0011010010;
        let flipped = TileVariation::flip_row(row);

        assert_eq!(0b0100101100, flipped);
    }
}