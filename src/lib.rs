#[macro_use]
extern crate nom;

mod types;
mod parser;
mod evaluator;
mod repl;
mod filereader;

use std::env;

pub fn main() {
    let args = env::args_os();
    if args.len() == 1 {
        println!(
            "No arguments provided. Starting the REPL...\n Use Ctrl+C to quit.",
        );
        repl::init();
    } else {
        //Assuming only one argument provided for now
        let file_name = args.into_iter().nth(1).unwrap();
        println!("Parsing file {:?} and outputting the results", file_name);
        let streamer = filereader::BCalcFileStreamer::new(file_name).unwrap();
        let mut env = types::Environment::new();
        for expr in streamer {
            println!("{}\n", types::display(evaluator::evaluate(&mut env, expr)));
        }
    }
}
