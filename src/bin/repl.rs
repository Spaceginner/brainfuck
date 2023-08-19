use std::io::{self, Write};
use brainfuck::{Machine, Program};


fn prompt(message: &str) -> String {
    print!("{message}");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer
}


fn main() {
    let mut machine = Machine::default();

    loop {
        // read the expression
        let expression = prompt("\n> ");

        // if CTRL + D was hit (EOF) - exit
        if expression.is_empty() {
            break;
        };

        // "compile" the expression and execute it
        machine.run(&Program::from(expression.as_str()));
    };
}
