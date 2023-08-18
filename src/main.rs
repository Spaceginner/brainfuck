mod interpreter;
mod program;


use crate::program::Program;


fn main() {
    println!("{:?}", Program::from("+++[-]"));
}
