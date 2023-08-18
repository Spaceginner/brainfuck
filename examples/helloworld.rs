use brainfuck::{Machine, Program};


const PROGRAM_SRC: &str = "-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[--->+<]>-----.---[->+++<]>.-[--->+<]>---.+++.------.--------.";


fn main() {
    let mut machine = Machine::default();
    let program = Program::from(PROGRAM_SRC);

    machine.run(&program);
}
