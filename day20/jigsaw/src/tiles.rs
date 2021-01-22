use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

const ROW_MASK: u32 = 0b1111111111;
const LEFT_EDGE_MASK: u32 = 0b1000000000;
const RIGHT_EDGE_MASK: u32 = 0b0000000001;

#[derive(Debug)]
pub struct Tile {
    pub id: u32,
    pub rows: Vec<u32>,
    pub edges: Vec<u32>
}

impl Tile {
    pub fn new(id: u32, rows: Vec<u32>) -> Tile {
        let edges = Tile::calculate_edges(&rows);
        Tile { id, rows, edges }
    }

    fn calculate_edges(rows: &Vec<u32>) -> Vec<u32> {
        let row_count = rows.len();

        let mut edges = Vec::new();

        let first_row = *rows.get(0).unwrap();
        let last_row = *rows.get(rows.len() - 1).unwrap();

        edges.push(first_row);
        edges.push(last_row);

        let mut left_edge = 0u32;
        let mut right_edge = 0u32;

        for (i, row) in rows.iter().enumerate() {
            let row = *row;

            left_edge = (left_edge << 1) | ((row & LEFT_EDGE_MASK) >> 9);
            right_edge = (right_edge << 1) | ((row & RIGHT_EDGE_MASK));
        }

        edges.push(left_edge);
        edges.push(right_edge);

        edges
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
            _ => "error"
        })
    }
}

impl Error for ParseTileError { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tile: Tile = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###".parse::<Tile>().unwrap();

        println!("{:?}", tile);

        assert_eq!(2311, tile.id);
        assert_eq!(vec![
            0b0011010010,
            0b0011100111,
            0b0111110010,
            0b0001011001
        ], tile.edges);
    }

    #[test]
    fn flip_row() {
        let row = 0b0011010010;
        let flipped = Tile::flip_row(row);

        assert_eq!(0b0100101100, flipped);
    }
}