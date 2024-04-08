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


#[derive(Debug)]
pub enum MachineException {
    InvalidInstructionPointer, NoProgramLoaded, NoOpenLoopsToBeClosed, EndOfProgram, UnclosedLoopsDetected
}


impl Machine {
    pub fn load(&mut self, program: Program) {
        self.program = Some(program);
    }
    
    pub fn unload(&mut self) {
        self.program = None;
        self.app_pointer = 0;
    }

    pub fn step(&mut self) -> Result<(), MachineException> {
        use Instruction as I;
        match &self.program {
            None => Err(MachineException::NoProgramLoaded),
            Some(program) => {
                match program.get(self.app_pointer) {
                    None => {
                        if self.app_pointer != program.get_instructions().len() {
                            Err(MachineException::InvalidInstructionPointer)
                        } else if !self.stack.is_empty() {
                            Err(MachineException::UnclosedLoopsDetected)
                        } else {
                            Err(MachineException::EndOfProgram)
                        }
                    },
                    Some(instruction) => {
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
                    }
                }
            }
        }
    }

    pub fn run(&mut self, program: Program) -> Option<MachineException> {
        self.load(program);
        
        loop {
            match self.step() {
                Ok(()) => continue,
                Err(MachineException::EndOfProgram) => {
                    self.unload();
                    break None;
                },
                Err(machine_exc) => break Some(machine_exc)
            }
        }
    }
}
