mod instruction;

pub use instruction::Instruction;

#[derive(Debug, Default)]
pub struct Program(Vec<Instruction>);


impl Program {
    pub fn get_instructions<'a>(&'a self) -> &'a[Instruction] {
        &self.0
    }
}

impl From<&str> for Program {
    fn from(code: &str) -> Self {
        let mut instructions = Vec::new();

        let mut skip = 0;
        for (i, c) in code.chars().enumerate() {
            if skip != 0 {
                skip -= 1;
                continue;
            };

            match c {
                '[' => {
                    // find the end of the loop
                    let loop_end = 'search: loop {
                        let mut deepness = 1_u64;

                        for (loop_i, loop_c) in code.chars().enumerate().skip(i + 1) {
                            match loop_c {
                                '[' => deepness += 1,
                                ']' => deepness -= 1,
                                _ => (),
                            };

                            if deepness == 0 {
                                break 'search loop_i;
                            };
                        };
                    };

                    instructions.push(Instruction::Loop(Self::from(&code[(i + 1)..loop_end])));

                    skip = loop_end - i;
                },
                _ => {
                    if let Ok(instruction) = Instruction::try_from(c) {
                        instructions.push(instruction);
                    };
                },
            }
        }

        Self(instructions)
    }
}
