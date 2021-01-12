use core::time;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct Schedule {
    buses: Vec<BusId>
}

impl Schedule {
    pub fn find_earliest_time(&self) -> usize {
        let (offset, max_bus_id) = self.max_bus_id().unwrap();

        let mut n = 0;
        let mut t = max_bus_id - offset;
        loop {
            // let t = max_bus_id * n - offset;
            if n % 1000 == 0 {
                println!("{}: {}", n, t);
            }
            
            let max_remainder = self.check_timestamp(t);
            if max_remainder == 0 {
                println!("{}: {}", n, t);
                return t;
            }            

            t = t + max_bus_id;
            n = n + 1;
        }
    }

    fn max_bus_id(&self) -> Option<(usize, usize)> {
        let mut max_id: Option<(usize, usize)> = None;

        for i in 0..self.buses.len() {
            if let BusId::Specific(specific_bus_id) = self.buses[i] {
                if let Some((_, max_id_value)) = max_id {
                    if specific_bus_id > max_id_value {
                        max_id = Some((i, specific_bus_id));
                    }
                } else {
                    max_id = Some((i, specific_bus_id));
                }
            } 
        }

        max_id
    }
    
    fn check_timestamp(&self, timestamp: usize) -> usize {
        let mut max_remainder = 0;

        for i in 0..self.buses.len() {
            if let BusId::Specific(bus_id) = self.buses[i] {
                let remainder = (timestamp + i) % bus_id;
                if remainder > max_remainder {
                    max_remainder = remainder;
                }
            }
        }

        max_remainder
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
    Specific(usize),
    Any
}

impl FromStr for BusId {
    type Err = ParseBusIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "x" {
            Ok(BusId::Any)
        } else {
            let parsed_id = s.parse::<usize>()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_timestamp() {
        let schedule = "7,13,x,x,59,x,31,19".parse::<Schedule>().unwrap();

        assert_eq!(0, schedule.check_timestamp(1068781));
    }
}