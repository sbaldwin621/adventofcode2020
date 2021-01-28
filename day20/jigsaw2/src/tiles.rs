use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

const LEFT_EDGE_MASK: u32 = 0b1000000000;
const RIGHT_EDGE_MASK: u32 = 0b0000000001;

const SEAMONSTER_0: u128 = 0b00000000000000000010;
const SEAMONSTER_1: u128 = 0b10000110000110000111;
const SEAMONSTER_2: u128 = 0b01001001001001001000;

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

    pub fn get_completed_puzzle(&self) -> Result<CompletedPuzzle, PuzzleError> {
        let mut result = Vec::new();

        let mut remaining_pieces = HashSet::new();
        for (id, _) in self.tiles.iter() {
            remaining_pieces.insert(*id);
        }

        let topleft = self.find_top_left_corner()
            .ok_or(PuzzleError::CouldntFindTopLeftCorner)?;
        
        remaining_pieces.remove(&topleft.id);
        result.push(topleft.clone());

        // Top edge
        for x in 1..self.size {
            let piece_to_left = result.last().unwrap();
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

            result.push(matching_variation.clone());
        }

        for y in 1..self.size {
            for x in 0..self.size {
                let piece_to_left = if x > 0 {
                    result.get((y * self.size + x - 1) as usize)
                } else {
                    None
                };
                let piece_above = result.get(((y - 1) * self.size + x) as usize).unwrap();

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
    
                result.push(matching_variation.clone());
            }
        }

        Ok(CompletedPuzzle::new(result, self.size))
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

#[derive(Debug, Clone)]
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

pub struct CompletedPuzzle {
    pub tiles: Vec<TileVariation>,
    pub size: usize,
    pub rows: Vec<u128>,
    pub monster_map: Vec<u128>,
    pub monster_count: usize
}

impl CompletedPuzzle {
    pub fn new(tiles: Vec<TileVariation>, size: usize) -> CompletedPuzzle {
        let rows = CompletedPuzzle::calculate_rows(&tiles, size);
        let (monster_map, monster_count) = CompletedPuzzle::map_sea_monsters(size, &rows);

        CompletedPuzzle { tiles, size, rows, monster_map, monster_count }
    }
    
    pub fn rotate(&self) -> CompletedPuzzle {
        let mut result = Vec::new();

        for y in 0..self.size {
            for x in 0..self.size {
                let new_y = (self.size - x) - 1;
                let new_x = y;

                let tile = self.tiles.get(new_y * self.size + new_x).unwrap();
                let rotated_tile = tile.rotate();

                result.push(rotated_tile);
            }
        }

        CompletedPuzzle::new(result, self.size)
    }

    pub fn get_roughness_score(&self) -> usize {
        let mut result = 0;

        for (row, monster_row) in self.rows.iter().zip(self.monster_map.iter()) {
            let row = *row;
            let monster_row = *monster_row;

            for n in (0..self.size * 8).rev() {
                let is_monster = (monster_row & (1 << n)) >> n == 1;
                let is_rough = (row & (1 << n)) >> n == 1;

                if is_rough && !is_monster {
                    result += 1;
                }
            }
        }

        result
    }

    fn calculate_rows(tiles: &Vec<TileVariation>, size: usize) -> Vec<u128> {
        const ROW_MASK: u32 = 0b0111111110;

        let mut result = Vec::new();

        for y in 0..size {
            let mut new_row0 = 0u128;
            let mut new_row1 = 0u128;
            let mut new_row2 = 0u128;
            let mut new_row3 = 0u128;
            let mut new_row4 = 0u128;
            let mut new_row5 = 0u128;
            let mut new_row6 = 0u128;
            let mut new_row7 = 0u128;

            for x in 0..size {
                let tile = tiles.get(y * size + x).unwrap();

                let row0 = ((tile.rows.get(1).unwrap() & ROW_MASK) >> 1) as u128;
                let row1 = ((tile.rows.get(2).unwrap() & ROW_MASK) >> 1) as u128;
                let row2 = ((tile.rows.get(3).unwrap() & ROW_MASK) >> 1) as u128;
                let row3 = ((tile.rows.get(4).unwrap() & ROW_MASK) >> 1) as u128;
                let row4 = ((tile.rows.get(5).unwrap() & ROW_MASK) >> 1) as u128;
                let row5 = ((tile.rows.get(6).unwrap() & ROW_MASK) >> 1) as u128;
                let row6 = ((tile.rows.get(7).unwrap() & ROW_MASK) >> 1) as u128;
                let row7 = ((tile.rows.get(8).unwrap() & ROW_MASK) >> 1) as u128;

                new_row0 |= row0 << ((size - x - 1) * 8);
                new_row1 |= row1 << ((size - x - 1) * 8);
                new_row2 |= row2 << ((size - x - 1) * 8);
                new_row3 |= row3 << ((size - x - 1) * 8);
                new_row4 |= row4 << ((size - x - 1) * 8);
                new_row5 |= row5 << ((size - x - 1) * 8);
                new_row6 |= row6 << ((size - x - 1) * 8);
                new_row7 |= row7 << ((size - x - 1) * 8);
            }

            result.push(new_row0);
            result.push(new_row1);
            result.push(new_row2);
            result.push(new_row3);
            result.push(new_row4);
            result.push(new_row5);
            result.push(new_row6);
            result.push(new_row7);
        }

        result
    }

    fn map_sea_monsters(size: usize, rows: &Vec<u128>) -> (Vec<u128>, usize) {
        let mut monster_map = rows.iter()
            .map(|_| 0u128)
            .collect::<Vec<_>>();

        let mut count = 0;

        for y in 0..rows.len() - 2 {
            for x in 0..size * 8 - 20 {
                let shifted_seamonster0 = SEAMONSTER_0 << x;
                let shifted_seamonster1 = SEAMONSTER_1 << x;
                let shifted_seamonster2 = SEAMONSTER_2 << x;

                let row = rows[y];
                if row & shifted_seamonster0 == shifted_seamonster0 {
                    let next_row = rows[y + 1];
                    if next_row & shifted_seamonster1 == shifted_seamonster1 {
                        let next_next_row = rows[y + 2];
                        if next_next_row & shifted_seamonster2 == shifted_seamonster2 {
                            count += 1;

                            monster_map[y] = shifted_seamonster0;
                            monster_map[y + 1] = shifted_seamonster1;
                            monster_map[y + 2] = shifted_seamonster2;
                        }
                    }
                }

            }
        }

        (monster_map, count)
    }
}

impl Display for CompletedPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (row, monster_row) in self.rows.iter().zip(self.monster_map.iter()) {
            let row = *row;
            let monster_row = *monster_row;

            for n in (0..self.size * 8).rev() {
                let is_monster = (monster_row & (1 << n)) >> n == 1;
                let is_rough = (row & (1 << n)) >> n == 1;

                let c = if is_monster { 
                    'O'
                } else if is_rough {
                    '#'
                } else {
                    '.'
                };

                result.push(c);
            }

            // for n in (0..self.size).rev() {
            //     let segment = (row >> (n * 8)) & SEGMENT_MASK;


            //     let segment_string = format!("{:08b}", segment)
            //         .replace("0", ".")
            //         .replace("1", "#");
                

            //     result.push_str(&segment_string);
            // }

            result.push_str("\n");
        }

        writeln!(f, "{}", result)
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