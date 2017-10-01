#[macro_use]
extern crate nom;

mod types;
mod parser;
mod evaluator;
#[cfg(test)]
mod tests;

pub fn main() {
    println!("Hello, world!");
}
