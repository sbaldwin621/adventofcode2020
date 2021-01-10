use crate::instructions::{Instruction, InstructionCommand};

#[derive(Debug)]
pub struct Ship {
    x: isize,
    y: isize,
    direction: ShipDirection
}

impl Ship {
    pub fn new() -> Ship {
        Ship { x: 0, y: 0, direction: ShipDirection::East }
    }

    pub fn apply(&mut self, instruction: &Instruction) {
        let value = instruction.value;

        match instruction.command {
            InstructionCommand::North => { self.y = self.y + value; }
            InstructionCommand::South => { self.y = self.y - value; }
            InstructionCommand::East => { self.x = self.x + value; }
            InstructionCommand::West => { self.x = self.x - value; }
            InstructionCommand::Left => { self.direction = self.direction.rotate(-value) }
            InstructionCommand::Right => { self.direction = self.direction.rotate(value) }
            InstructionCommand::Forward => { self.move_forward(value); }
        }
    }

    fn move_forward(&mut self, value: isize) {
        match self.direction {
            ShipDirection::North => { self.y = self.y + value; }
            ShipDirection::South => { self.y = self.y - value; }
            ShipDirection::East => { self.x = self.x + value; }
            ShipDirection::West => { self.x = self.x - value; }
        }
    }

    pub fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShipDirection { 
    North,
    South,
    East,
    West
}

impl ShipDirection {
    pub fn rotate(&self, amount: isize) -> ShipDirection {
        let mut result = *self;
        let mut amount_remaining = amount;

        if (amount > 0) {
            while amount_remaining > 0 {
                amount_remaining = amount_remaining - 90;
                result = result.rotate_right();
            }

            result
        } else {
            while amount_remaining < 0 {
                amount_remaining = amount_remaining + 90;
                result = result.rotate_left();
            }

            result
        }
    }

    pub fn rotate_right(&self) -> ShipDirection {
        match self {
            ShipDirection::North => ShipDirection::East,
            ShipDirection::South => ShipDirection::West,
            ShipDirection::East => ShipDirection::South,
            ShipDirection::West => ShipDirection::North
        }
    }

    pub fn rotate_left(&self) -> ShipDirection {
        match self {
            ShipDirection::North => ShipDirection::West,
            ShipDirection::South => ShipDirection::East,
            ShipDirection::East => ShipDirection::North,
            ShipDirection::West => ShipDirection::South
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right() {
        let result = ShipDirection::North.rotate_right().rotate_right().rotate_right();

        assert_eq!(ShipDirection::West, result);
    }

    #[test]
    fn rotate_left() {
        let result = ShipDirection::North.rotate_left().rotate_left().rotate_left();

        assert_eq!(ShipDirection::East, result);
    }

    #[test]
    fn rotate_270() {
        let result = ShipDirection::North.rotate(270);
        
        assert_eq!(ShipDirection::West, result);
    }

    #[test]
    fn rotate_neg_270() {
        let result = ShipDirection::North.rotate(-270);
        
        assert_eq!(ShipDirection::East, result);
    }
}