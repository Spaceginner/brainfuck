#[derive(Debug)]
enum Instruction {
    MoveRight, MoveLeft,
    Increment, Decrement,
    Loop(Program),
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
            _ => Err(()),
        }
    }
}


#[derive(Debug, Default)]
pub struct Program(Vec<Instruction>);


impl From<&str> for Program {
    fn from(code: &str) -> Self {
        let mut instructions = Vec::new();

        let mut skip = 0;
        for (i, c) in dbg!(code).chars().enumerate() {
            if skip != 0 {
                skip -= 1;
                continue;
            };

            match dbg!(c) {
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
