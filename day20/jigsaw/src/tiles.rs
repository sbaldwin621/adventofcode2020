use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

const LEFT_EDGE_MASK: u32 = 0b1000000000;
const RIGHT_EDGE_MASK: u32 = 0b0000000001;

#[derive(Debug)]
pub struct Tileset {
    pub tiles: HashMap<u32, Tile>,
    edge_map: HashMap<u32, Vec<u32>>
}

impl Tileset {
    pub fn new(tiles: Vec<Tile>) -> Tileset {
        let mut tile_map = HashMap::new();

        for tile in tiles {
            tile_map.insert(tile.id, tile);
        }

        let edge_map = Tileset::get_edge_map(&tile_map);

        println!("{:?}", edge_map);

        Tileset { tiles: tile_map, edge_map }
    }

    pub fn find_corners(&self) -> Vec<u32> {
        let mut corners = Vec::new();

        for (id, tile) in self.tiles.iter() {
            let mut max_shared_edge_count = 0;

            for variation in tile.variations.iter() {
                let mut shared_edge_count = 0;
                for edge in variation.edges().iter() {
                    let tile_count = self.edge_map.get(edge).unwrap().len() - 1;
                    if tile_count > 0 {
                        shared_edge_count += 1;
                    }
                }

                if shared_edge_count > max_shared_edge_count {
                    max_shared_edge_count = shared_edge_count;
                }
            }

            if max_shared_edge_count == 2 {
                corners.push(*id);
            }
        }

        corners
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
}

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
        let (top_edge, right_edge, bottom_edge, left_edge) = Tile::calculate_edges(&rows);

        let mut variations = Vec::new();

        // Normal
        let original = TileVariation::new(id, top_edge, right_edge, bottom_edge, left_edge);
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
    pub top_edge: u32,
    pub right_edge: u32,
    pub bottom_edge: u32,
    pub left_edge: u32
}

impl TileVariation {
    pub fn new(id: u32, top_edge: u32, right_edge: u32, bottom_edge: u32, left_edge: u32) -> TileVariation {
        TileVariation { id, top_edge, right_edge, bottom_edge, left_edge }
    }

    pub fn flip(&self) -> TileVariation {
        let top_edge = self.top_edge;
        let right_edge = self.left_edge;
        let bottom_edge = self.bottom_edge;
        let left_edge = self.right_edge;

        TileVariation::new(self.id, top_edge, right_edge, bottom_edge, left_edge)
    }

    pub fn rotate(&self) -> TileVariation {
        let top_edge = TileVariation::flip_row(self.left_edge);
        let right_edge = self.top_edge;
        let bottom_edge = TileVariation::flip_row(self.right_edge);
        let left_edge = self.bottom_edge;

        TileVariation::new(self.id, top_edge, right_edge, bottom_edge, left_edge)
    }
    
    pub fn edges(&self) -> [u32; 4] {
        [self.top_edge, self.right_edge, self.bottom_edge, self.left_edge]
    }

    fn flip_row(row: u32) -> u32 {
        row.reverse_bits() >> 22
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
        let flipped = Tile::flip_row(row);

        assert_eq!(0b0100101100, flipped);
    }
}