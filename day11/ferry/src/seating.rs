use std::error::Error;
use std::fmt::Display;
use std::iter;
use std::str::FromStr;

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

    pub fn step(&mut self) {
        todo!()
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

#[derive(Debug, Clone)]
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