use std::error::Error;
use std::fmt::Display;
use std::iter;
use std::str::FromStr;
use std::time::UNIX_EPOCH;

#[derive(Debug)]
pub struct SeatingChart {
    rows: Vec<SeatingChartRow>,
    swap: Vec<SeatingChartRow>
}

impl SeatingChart {
    pub fn new(rows: Vec<SeatingChartRow>) -> Result<SeatingChart, CreateSeatingChartError> {
        let first_row = rows.get(0)
            .ok_or(CreateSeatingChartError::ArgumentEmpty)?;

        let chart_width = first_row.len();
        for row in &rows {
            if row.len() != chart_width {
                return Err(CreateSeatingChartError::UnevenRows);
            }
        }

        let mut swap = Vec::new();
        for i in 0..rows.len() {
            let row = iter::repeat(SeatingChartStatus::Floor)
                .take(chart_width)
                .collect::<Vec<_>>();

            swap.push(SeatingChartRow { seats: row });
        }

        Ok(SeatingChart { rows, swap })
    }

    // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    // Otherwise, the seat's state does not change.
    pub fn step(&mut self) -> usize {
        let mut change_count = 0;

        for y in 0..self.rows.len() {
            let row = &self.rows[y];
            
            for x in 0..row.len() {
                let seat = &row.seats[x];
                
                let nw = self.value_by_direction(x, y, -1, -1);
                let n  = self.value_by_direction(x, y, 0, -1);
                let ne  = self.value_by_direction(x, y, 1, -1);
                let w  = self.value_by_direction(x, y, -1, 0);
                let e  = self.value_by_direction(x, y, 1, 0);
                let sw  = self.value_by_direction(x, y, -1, 1);
                let s  = self.value_by_direction(x, y, 0, 1);
                let se  = self.value_by_direction(x, y, 1, 1);
                
                let occupied_neighbors = nw + n + ne + w + e + sw + s + se;

                let next_status = match seat {
                    SeatingChartStatus::Unoccupied => {
                        if occupied_neighbors == 0 {
                            SeatingChartStatus::Occupied
                        } else {
                            SeatingChartStatus::Unoccupied
                        }
                    },
                    SeatingChartStatus::Occupied => {
                        if occupied_neighbors >= 5 {
                            SeatingChartStatus::Unoccupied
                        } else {
                            SeatingChartStatus::Occupied
                        }
                    },
                    _ => { seat.clone() }
                };

                if seat != &next_status {
                    change_count = change_count + 1;
                }

                self.swap[y].seats[x] = next_status;
            }
        }

        std::mem::swap(&mut self.rows, &mut self.swap);

        change_count
    }

    fn get_by_coords(&self, x: usize, y: usize) -> Option<&SeatingChartStatus> {
        self.rows.get(y)?.seats.get(x)
    }

    fn value_by_direction(&self, x: usize, y: usize, x_dir: isize, y_dir: isize) -> usize {
        if (x == 0 && x_dir == -1) || (y == 0 && y_dir == -1) {
            return 0;
        }

        let neighbor_x: usize = ((x as isize) + x_dir) as usize;
        let neighbor_y: usize = ((y as isize) + y_dir) as usize;

        if let Some(status) = self.get_by_coords(neighbor_x, neighbor_y) {
            match status {
                SeatingChartStatus::Occupied => 1,
                SeatingChartStatus::Unoccupied => 0,
                SeatingChartStatus::Floor => self.value_by_direction(neighbor_x, neighbor_y, x_dir, y_dir)
            }
        } else {
            0
        }
    }

    pub fn occupied_count(&self) -> usize {
        let mut count = 0;
        for row in &self.rows { 
            for seat in &row.seats {
                if seat == &SeatingChartStatus::Occupied {
                    count = count + 1;
                }
            }
        }
        
        count
    }
}

impl Display for SeatingChart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        let row_count = self.rows.len();
        if row_count > 0 {
            for i in 0..row_count-1 {
                result.push_str(&format!("{}\n", self.rows[i]));
            }
            
            result.push_str(&format!("{}", self.rows[row_count - 1]));
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub enum CreateSeatingChartError {
    ArgumentEmpty,
    UnevenRows
}

impl Display for CreateSeatingChartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            CreateSeatingChartError::ArgumentEmpty => "argument cannot be empty",
            CreateSeatingChartError::UnevenRows => "rows must be of equal width"
        })
    }
}

impl Error for CreateSeatingChartError { }

#[derive(Debug)]
pub enum CalculateSeatingChartStepError {
    ArgumentEmpty,
    UnevenRows
}

impl Display for CalculateSeatingChartStepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            CalculateSeatingChartStepError::ArgumentEmpty => "argument cannot be empty",
            CalculateSeatingChartStepError::UnevenRows => "rows must be of equal width"
        })
    }
}

impl Error for CalculateSeatingChartStepError { }

#[derive(Debug)]
pub struct SeatingChartRow {
    seats: Vec<SeatingChartStatus>
}

impl SeatingChartRow {
    pub fn len(&self) -> usize {
        self.seats.len()
    }
}

impl Display for SeatingChartRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for seat in &self.seats {
            result.push(match seat {
                SeatingChartStatus::Unoccupied => 'L',
                SeatingChartStatus::Occupied => '#',
                SeatingChartStatus::Floor => '.'
            });
        }

        write!(f, "{}", result)
    }
}

impl FromStr for SeatingChartRow {
    type Err = ParseSeatingChartRowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seats = Vec::new();

        for c in s.chars() {
            let status = match c {
                'L' => SeatingChartStatus::Unoccupied,
                '#' => SeatingChartStatus::Occupied,
                '.' => SeatingChartStatus::Floor,
                _ => { return Err(ParseSeatingChartRowError::UnknownCharacter(c)) }
            };

            seats.push(status);
        }

        Ok(SeatingChartRow { seats })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SeatingChartStatus {
    Unoccupied,
    Occupied,
    Floor
}

#[derive(Debug)]
pub enum ParseSeatingChartRowError {
    UnknownCharacter(char)
}

impl Display for ParseSeatingChartRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseSeatingChartRowError::UnknownCharacter(c) => format!("unknown character '{}'", c)
        })
    }
}

impl Error for ParseSeatingChartRowError { }