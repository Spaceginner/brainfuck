use std::io::{self, Write};
use brainfuck::{Machine, Program};


fn prompt(message: &str) -> Result<String, io::Error> {
    print!("{message}");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}


fn main() {
    let mut machine = Machine::default();

    loop {
        // read the expression
        let expression = prompt("\n> ").unwrap_or_default();

        // if CTRL + D was hit (EOF) - exit
        if expression.is_empty() {
            break;
        };

        // "compile" the expression and execute it
        machine.run(Program::from(expression.as_str()));
    };
}
