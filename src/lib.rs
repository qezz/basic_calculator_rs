#[macro_use]
extern crate nom;

mod types;
mod parser;
mod evaluator;
mod repl;
mod filereader;

pub fn main() {
    let file_name = "/home/balaji/Projects/rust/basic_calculator/00-sample.bc";
    let streamer = filereader::BCalcFileStreamer::new(file_name).unwrap();
    let mut env = types::Environment::new();
    for expr in streamer {
        println!("{}\n", display(evaluator::evaluate(&mut env, expr)));
    }
}

fn display(r: types::MyResult) -> String {
    match r {
        Ok(value) => value.to_string(),
        Err(error) => error.to_string(),
    }
}
