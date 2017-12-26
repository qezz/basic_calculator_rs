use std::io::{self, Write};
use parser::*;
use types::Environment;
use types::MyResult;
use evaluator::*;

pub fn init() {
    let mut environment = Environment::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let (_ignore, parsed) = expr(&input[..]).unwrap();
        let result = format!("{}\n", display(evaluate(&mut environment, parsed)));
        io::stdout().write(result.to_string().as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
}

fn display(r: MyResult) -> String {
    match r {
        Ok(value) => value.to_string(),
        Err(error) => error.to_string(),
    }
}
