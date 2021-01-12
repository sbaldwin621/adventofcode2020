use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct Schedule {
    buses: Vec<BusId>
}

impl Schedule {
    // 17,x,13,19 is 3417

    // x = 0 (mod 17) [17 - 0]
    // x = 11 (mod 13) [13 - 2]
    // x = 16 (mod 19) [19 - 3]
    // x = 3417 + 4199k
    
    // 17 * 13 * 19 = 
    
    // 16 mod 19 = 16, 35, 54
    

    pub fn find_earliest_time(&self) -> i64 {
        let product = self.buses.iter().fold(1, |accum, bus| match bus { 
            BusId::Specific(bus_id) => accum * bus_id,
            _ => accum
        });

        let n = self.buses.iter().enumerate().filter_map(|(i, bus)| match bus { 
            BusId::Specific(bus_id) => Some(*bus_id),
            _ => None
        }).collect::<Vec<_>>();

        let a = self.buses.iter().enumerate().filter_map(|(i, bus)| match bus { 
            BusId::Specific(bus_id) => Some((*bus_id - (i as i64)) % bus_id),
            _ => None
        }).collect::<Vec<_>>();

        let chinese_remainder = Schedule::chinese_remainder_euclid(n, product, a);

        let result;
        if chinese_remainder > 0 {
            result = chinese_remainder % product;
        } else {
            result = product + (chinese_remainder % product);
        }
        // 1068781

        println!("{}", result);

        todo!()
    }

    // fn max_bus_id(&self) -> Option<(usize, usize)> {
    //     let mut max_id: Option<(usize, usize)> = None;

    //     for i in 0..self.buses.len() {
    //         if let BusId::Specific(specific_bus_id) = self.buses[i] {
    //             if let Some((_, max_id_value)) = max_id {
    //                 if specific_bus_id > max_id_value {
    //                     max_id = Some((i, specific_bus_id));
    //                 }
    //             } else {
    //                 max_id = Some((i, specific_bus_id));
    //             }
    //         } 
    //     }

    //     max_id
    // }
    
    // fn check_timestamp(&self, timestamp: usize) -> usize {
    //     let mut max_remainder = 0;

    //     for i in 0..self.buses.len() {
    //         if let BusId::Specific(bus_id) = self.buses[i] {
    //             let remainder = (timestamp + i) % bus_id;
    //             if remainder > max_remainder {
    //                 max_remainder = remainder;
    //             }
    //         }
    //     }

    //     max_remainder
    // }

    /*
    def ChineseRemainderEuclid(n, N, a):
    result = 0

    for i in range(len(n)):
        ai = a[i]
        ni = n[i]

        _, _, si = ExtendedEuclid(ni, N // ni)
        result += ai * si * (N // ni)

    return LeastPositiveEquivalent(result, N)
    */
    fn chinese_remainder_euclid(n: Vec<i64>, N: i64, a: Vec<i64>) -> i64 {
        let mut result = 0;

        for i in 0..n.len() {
            let ai = a[i];
            let ni = n[i];

            let (_, _, si) = Schedule::extended_euclid(ni, N / ni);
            result = result + ai * si * (N / ni);
        }

        result
    }

    /*
    def ExtendedEuclid(x, y):
    x0, x1, y0, y1 = 1, 0, 0, 1

    while y > 0:
        q, x, y = math.floor(x / y), y, x % y
        x0, x1 = x1, x0 - q * x1
        y0, y1 = y1, y0 - q * y1

    return q, x0, y0  # gcd and the two coefficients
    */
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

    fn invmod(a: i64, m: i64) -> i64 {
        let (g, x, y) = Schedule::extended_euclid(a, m);

        if g != 1 {
            panic!("modular inverse does not exist");
        }
        
        x % m
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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn check_timestamp() {
    //     let schedule = "7,13,x,x,59,x,31,19".parse::<Schedule>().unwrap();

    //     assert_eq!(0, schedule.check_timestamp(1068781));
    // }
}