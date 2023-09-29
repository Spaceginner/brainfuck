#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    MoveRight, MoveLeft,
    Increment, Decrement,
    LoopEnter, LoopExit,
    Get, Put,
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::MoveRight),
            '<' => Ok(Self::MoveLeft),
            '+' => Ok(Self::Increment),
            '-' => Ok(Self::Decrement),
            ',' => Ok(Self::Get),
            '.' => Ok(Self::Put),
            '[' => Ok(Self::LoopEnter),
            ']' => Ok(Self::LoopExit),
            _ => Err(()),
        }
    }
}
