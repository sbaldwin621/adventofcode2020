use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;
use std::fmt::{Display, Write};
use std::mem::swap;

#[derive(Debug)]
pub struct Cube {
    current: PointSet,
    next: PointSet
}

impl Cube {
    pub fn new(current: PointSet) -> Cube {
        let next = PointSet::new();

        Cube { current, next }
    }

    pub fn iter(&self) -> Iter<Point> {
        self.current.iter()
    }

    pub fn len(&self) -> usize {
        self.current.len()
    }

    // - If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
    //   Otherwise, the cube becomes inactive.
    // - If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
    //   Otherwise, the cube remains inactive.
    pub fn step(&mut self) {
        self.next.clear();

        let mut points_neighboring_active = HashMap::new();
        
        for point in self.current.iter() {
            let mut neighbor_count = 0;

            for neighbor in point.get_neighbors() {                
                if self.current.contains(&neighbor) {
                    neighbor_count = neighbor_count + 1;
                }

                let new_count = if let Some(count) = points_neighboring_active.get(&neighbor) {
                    count + 1
                } else {
                    1
                };

                points_neighboring_active.insert(neighbor, new_count);
            }

            if neighbor_count == 2 || neighbor_count == 3 {
                self.next.insert(point.clone());
            }
        }

        for (point, count) in points_neighboring_active {
            if count == 3 {
                self.next.insert(point);
            }
        }

        swap(&mut self.current, &mut self.next);
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.current)
    }
}

impl From<Vec<(i64, i64, i64)>> for Cube {
    fn from(vec: Vec<(i64, i64, i64)>) -> Self {
        let mut current = PointSet::new();
        for (x, y, z) in vec {
            current.insert(Point::new(x, y, z));
        }

        Cube::new(current)
    }
}

#[derive(Debug)]
pub struct PointSet {
    points: HashSet<Point>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64
}

impl PointSet {
    pub fn new() -> PointSet {
        let points = HashSet::new();
        PointSet { points, min_x: 0, max_x: 0, min_y: 0, max_y: 0, min_z: 0, max_z: 0 }
    }

    pub fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    pub fn insert(&mut self, point: Point) {
        if point.x < self.min_x {
            self.min_x = point.x;
        } else if point.x > self.max_x {
            self.max_x = point.x;
        }

        if point.y < self.min_y {
            self.min_y = point.y;
        } else if point.y > self.max_y {
            self.max_y = point.y;
        }

        if point.z < self.min_z {
            self.min_z = point.z;
        } else if point.z > self.max_z {
            self.max_z = point.z;
        }

        self.points.insert(point);
    }
    
    pub fn clear(&mut self) {
        self.points.clear();

        self.min_x = 0;
        self.max_x = 0;
        self.min_y = 0;
        self.max_y = 0;
        self.min_z = 0;
        self.max_z = 0;
    }
}

impl Display for PointSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for z in self.min_z..=self.max_z {
            writeln!(&mut result, "z={}", z)?;

            for y in self.min_y..=self.max_y {
                for x in self.min_x..=self.max_x {
                    write!(&mut result, "{}", if self.contains(&Point::new(x, y, z)) {
                        "#"
                    } else {
                        "."
                    })?;
                }

                writeln!(&mut result, "")?;
            }
        }

        writeln!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    #[inline]
    pub fn x(&self) -> i64 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> i64 {
        self.y
    }

    #[inline]
    pub fn z(&self) -> i64 {
        self.z
    }

    pub fn get_neighbors(&self) -> Vec<Point> {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        let mut neighbors = Vec::new();

        for x_delta in -1..2 {
            for y_delta in -1..2 {
                for z_delta in -1..2 {
                    if x_delta != 0 || y_delta != 0 || z_delta != 0 {
                        neighbors.push(Point::new(x + x_delta, y + y_delta, z + z_delta));
                    }
                }
            }
        }

        neighbors
    }
}