mod memory;

use std::collections::VecDeque;
use std::io::{self, Read, Write};

use crate::program::{Program, Instruction};
use memory::Memory;

#[derive(Debug, Default)]
pub struct Machine {
    memory: Memory,
    mem_pointer: isize,
    app_pointer: usize,
    program: Option<Program>,
    stack: VecDeque<usize>,
}


pub enum MachineException {
    InvalidInstructionPointer, NoProgramLoaded, NoOpenLoopsToBeClosed
}


impl Machine {
    pub fn reset(&mut self) {
        *self = Default::default();
    }

    pub fn load(&mut self, program: Program) {
        self.program = Some(program);
    }

    pub fn step(&mut self) -> Result<(), MachineException> {
        use Instruction as I;
        if let Some(program) = &self.program {
            if let Some(instruction) = program.get(self.app_pointer) {
                match instruction {
                    I::MoveRight => self.mem_pointer += 1,
                    I::MoveLeft => self.mem_pointer -= 1,
                    I::Increment => self.memory[self.mem_pointer] = self.memory[self.mem_pointer].wrapping_add(1),
                    I::Decrement => self.memory[self.mem_pointer] = self.memory[self.mem_pointer].wrapping_sub(1),
                    I::Get => {
                        let mut buffer = vec![0];
                        io::stdin().read_exact(&mut buffer).expect("failed to read");
                        self.memory[self.mem_pointer] = buffer[0];
                    }
                    I::Put => {
                        print!("{}", self.memory[self.mem_pointer] as char);
                        io::stdout().flush().expect("failed to flush stdout");
                    }
                    I::LoopEnter => {
                        if self.memory[self.mem_pointer] != 0 {
                            self.stack.push_front(self.app_pointer);
                        } else {
                            let mut loop_depth = 1;
                            for new_app_ptr in self.app_pointer.. {
                                match program.get(new_app_ptr) {
                                    Some(I::LoopEnter) => loop_depth += 1,
                                    Some(I::LoopExit) => loop_depth -= 1,
                                    Some(_) => continue,
                                    None => return Err(MachineException::NoOpenLoopsToBeClosed),
                                };
                                
                                if loop_depth == 0 {
                                    self.app_pointer = new_app_ptr;
                                    break;
                                }
                            }
                        };
                    }
                    I::LoopExit => {
                        if self.memory[self.mem_pointer] == 0 {
                            self.stack.pop_front();
                        } else {
                            match self.stack.front() {
                                Some(loop_start) => self.app_pointer = *loop_start,
                                None => return Err(MachineException::NoOpenLoopsToBeClosed)
                            }
                        };
                    }
                };

                self.app_pointer += 1;
                Ok(())
            } else {
                Err(MachineException::InvalidInstructionPointer)
            }
        } else {
            Err(MachineException::NoProgramLoaded)
        }
    }

    pub fn run(&mut self, program: Program) {
        self.load(program);
        while let Ok(()) = self.step() {};
    }
}
