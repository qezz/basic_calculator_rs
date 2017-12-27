#[macro_use]
extern crate nom;

mod types;
mod parser;
mod evaluator;
mod repl;
mod filereader;

pub fn main() {
    let file_name = "00-sample.bc";
    let streamer = filereader::BCalcFileStreamer::new(file_name).unwrap();
    let mut env = types::Environment::new();
    for expr in streamer {
        println!("{}\n", types::display(evaluator::evaluate(&mut env, expr)));
    }
}
