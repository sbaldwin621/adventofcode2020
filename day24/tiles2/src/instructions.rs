#[derive(Debug)]
pub enum Direction {
    West,
    Northwest,
    Northeast,
    East,
    Southeast,
    Southwest
}

#[derive(Debug)]
pub struct InstructionSet {
    instructions: Vec<Direction>
}

impl InstructionSet {
    pub fn new(instructions: Vec<Direction>) -> InstructionSet {
        InstructionSet { instructions }
    }

    pub fn iter(&self) -> std::slice::Iter<Direction> {
        self.instructions.iter()
    }
}

#[derive(Debug)]
pub struct PuzzleInput {
    pub instruction_sets: Vec<InstructionSet>
}

impl PuzzleInput {
    pub fn new(instruction_sets: Vec<InstructionSet>) -> PuzzleInput {
        PuzzleInput { instruction_sets }
    }
}