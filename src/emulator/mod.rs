mod memory;

use std::io::{self, Write};

use crate::program::{Program, Instruction};
use memory::Memory;

#[derive(Debug, Default)]
pub struct Machine {
    memory: Memory,
    pointer: usize,
}


impl Machine {
    pub fn reset(&mut self) {
        *self = Default::default();
    }

    pub fn run(&mut self, program: &Program) {
        for instruction in program.get_instructions().iter() {
            match instruction {
                Instruction::MoveRight => self.pointer += 1,
                Instruction::MoveLeft => self.pointer -= 1,
                Instruction::Increment => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
                Instruction::Decrement => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
                Instruction::Loop(subprogram) => {
                    while self.memory[self.pointer] != 0 {
                        self.run(subprogram);
                    };
                },
                Instruction::Get => todo!("Instruction::Get"),
                Instruction::Put => {
                    print!("{}", self.memory[self.pointer] as char);
                    io::stdout().flush().expect("failed to flush stdout");
                }
            }
        }
    }
}
