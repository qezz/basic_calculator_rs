use std::io::{self, Write};
use parser::parse;
use types::Environment;
use types::MyResult;
use evaluator::*;

pub fn init() {
    let mut environment = Environment::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let result = format!(
            "{}\n",
            display(parse(&input[..]).and_then(
                |expr| evaluate(&mut environment, expr),
            ))
        );
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
