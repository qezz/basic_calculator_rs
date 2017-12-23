use std::io::{self, Write};
use parser::*;
use types::Environment;
use evaluator::*;

pub fn init() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let (_ignore, parsed) = expr(&input[..]).unwrap();
        let result = format!("{}\n", evaluate(&mut Environment::new(), parsed).1);
        io::stdout().write(result.to_string().as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
}