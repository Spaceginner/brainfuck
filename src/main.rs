mod emulator;
mod program;


use crate::emulator::Machine;
use crate::program::Program;


const PROGRAM_SRC: &str = "-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[--->+<]>-----.---[->+++<]>.-[--->+<]>---.+++.------.--------.";


fn main() {
    let mut machine = Machine::default();

    let program = Program::from(PROGRAM_SRC);

    machine.run(&program);
}
