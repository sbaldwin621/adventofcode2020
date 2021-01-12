use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct Schedule {
    buses: Vec<BusId>
}

impl Schedule {
    /*
    Implementation of Chinese remainder theorem translated from Python as described here:
        https://shainer.github.io/crypto/math/2017/10/22/chinese-remainder-theorem.html

    Input:
    7,13,x,x,59,x,31,19
    
    Translates to:
    x = 0 (mod 7)   // index = 0
    x = 12 (mod 13) // index = 1, 13 - 1 = 12
    x = 55 (mod 59) // index = 4, 59 - 4 = 55
    x = 25 (mod 31) // index = 6, 31 - 6 = 25
    x = 12 (mod 19) // index = 7, 19 - 7 = 12

    Below algorithm gives result:
    1068781
    */
    pub fn find_earliest_time(&self) -> i64 {
        let product = self.buses.iter().fold(1, |accum, bus| match bus { 
            BusId::Specific(bus_id) => accum * bus_id,
            _ => accum
        });

        let mut result = 0;

        for (i, bus) in self.buses.iter().enumerate() {
            if let BusId::Specific(bus_id) = bus {
                let ai = (bus_id - (i as i64)) % bus_id;
                let ni = *bus_id;

                println!("x = {} (mod {})", ai, ni);
    
                let (_, _, si) = Schedule::extended_euclid(ni, product / ni);
                result = result + ai * si * (product / ni);
            }            
        }

        // Algorithm can give negative number, need minimum positive answer for problem
        if result > 0 {
            result % product
        } else {
            product + (result % product)
        }
    }

    fn extended_euclid(x: i64, y: i64) -> (i64, i64, i64) {
        let mut q = 0;

        let mut x = x;
        let mut y = y;

        let mut x0 = 1;
        let mut x1 = 0;
        let mut y0 = 0;
        let mut y1 = 1;

        while y > 0 {
            q = ((x as f64) / (y as f64)).floor() as i64;

            let swap = x;
            x = y;
            y = swap % y;

            let swap = x0;
            x0 = x1;
            x1 = swap - q * x1;

            let swap = y0;
            y0 = y1;
            y1 = swap - q * y1;
        }

        (q, x0, y0)
    }
}

impl FromStr for Schedule {
    type Err = ParseBusIdError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buses = Vec::new();
        let split = s.split(',');
        for element in split {
            let bus_id = element.parse::<BusId>()?;
            buses.push(bus_id);
        }

        Ok(Schedule { buses })
    }
}

#[derive(Debug)]
pub enum BusId {
    Specific(i64),
    Any
}

impl FromStr for BusId {
    type Err = ParseBusIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "x" {
            Ok(BusId::Any)
        } else {
            let parsed_id = s.parse::<i64>()
                .map_err(|_| ParseBusIdError::InvalidBusId)?;
            
            Ok(BusId::Specific(parsed_id))
        }
    }
}

#[derive(Debug)]
pub enum ParseBusIdError {
    InvalidBusId
}

impl Display for ParseBusIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseBusIdError::InvalidBusId => "not a valid bus ID"
        })
    }
}

impl Error for ParseBusIdError { }