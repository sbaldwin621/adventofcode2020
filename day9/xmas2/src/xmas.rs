use std::collections::HashSet;


pub fn find_invalid_number(data: &Vec<i64>, preamble_size: usize) -> Option<i64> {
    for i in preamble_size..data.len() {
        if !is_valid(&data, preamble_size, i) {
            return Some(data[i]);
        }
    }

    None
}

pub fn find_weakness(data: &Vec<i64>, target: i64) -> Option<i64> {
    let len = data.len();
    let mut start = 0;
    let mut end = 1;

    while start < len && end < len {
        let sum: i64 = data[start..end].iter().sum();
        if sum == target {
            let smallest = data[start..end].iter().min()?;
            let largest = data[start..end].iter().max()?;
            return Some(smallest + largest);
        } else if sum < target {
            end = end + 1;
        } else {
            start = start + 1;
            if end <= start { 
                end = start + 1;
            }
        }
    }
    
    None
}

fn is_valid(data: &Vec<i64>, preamble_size: usize, i: usize) -> bool {
    let mut set = HashSet::new();

    let num = data[i];
    for j in (i - preamble_size)..i {
        let other = data[j];
        let compliment = num - other;
        
        if set.contains(&compliment) {
            return true;
        } else {
            set.insert(other);
        }
    }
    
    false
}
