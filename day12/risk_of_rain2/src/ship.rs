use crate::instructions::{Instruction, InstructionCommand};

#[derive(Debug)]
pub struct Ship {
    x: isize,
    y: isize,
    waypoint_x: isize,
    waypoint_y: isize
}

impl Ship {
    pub fn new() -> Ship {
        Ship { x: 0, y: 0, waypoint_x: 10, waypoint_y: 1 }
    }

    pub fn apply(&mut self, instruction: &Instruction) {
        let value = instruction.value;

        match instruction.command {
            InstructionCommand::North => { self.waypoint_y = self.waypoint_y + value; }
            InstructionCommand::South => { self.waypoint_y = self.waypoint_y - value; }
            InstructionCommand::East => { self.waypoint_x = self.waypoint_x + value; }
            InstructionCommand::West => { self.waypoint_x = self.waypoint_x - value; }
            InstructionCommand::Left => { self.rotate(-value); }
            InstructionCommand::Right => { self.rotate(value); }
            InstructionCommand::Forward => { self.move_forward(value); }
        }
    }

    fn rotate(&mut self, value: isize) {
        let mut remaining_value = value;
        if value > 0 {
            while remaining_value > 0 {
                remaining_value = remaining_value - 90;

                let current_x = self.waypoint_x;
                let current_y = self.waypoint_y;

                self.waypoint_x = current_y;
                self.waypoint_y = -current_x;
            }
        } else {
            while remaining_value < 0 {
                remaining_value = remaining_value + 90;

                let current_y = self.waypoint_y;
                let current_x = self.waypoint_x;

                self.waypoint_x = -current_y;
                self.waypoint_y = current_x;
            }
        }
    }

    fn move_forward(&mut self, value: isize) {
        self.x = self.x + self.waypoint_x * value;
        self.y = self.y + self.waypoint_y * value;
    }

    pub fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}
