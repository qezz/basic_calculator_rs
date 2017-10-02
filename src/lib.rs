#[macro_use]
extern crate nom;

mod types;
mod parser;
mod evaluator;
mod repl;

pub fn main() {
    repl::init();
}
