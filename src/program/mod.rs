mod instruction;

use std::ops::Index;
pub use instruction::Instruction;

#[derive(Debug, Default, Clone)]
pub struct Program(Vec<Instruction>);


impl Program {
    pub fn get_instructions(&self) -> &[Instruction] {
        &self.0
    }

    pub fn get(&self, index: usize) -> Option<&Instruction> {
        self.0.get(index)
    }
}

impl Index<usize> for Program {
    type Output = Instruction;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<&str> for Program {
    fn from(code: &str) -> Self {
        let mut instructions = Vec::new();

        for (i, c) in code.chars().enumerate() {
            if let Ok(instruction) = Instruction::try_from(c) {
                instructions.push(instruction);
            };
        };

        Self(instructions)
    }
}
