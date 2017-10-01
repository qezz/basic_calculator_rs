#[macro_use]
extern crate nom;

use nom::digit;

#[derive(Debug, PartialEq)]
enum Expr {
    ENum(i32),
}

fn parse_num(str: &str) -> Expr {
    // forgoing all error handling for now
    let num: i32 = str.trim().parse().unwrap();
    Expr::ENum(num)
}

named!(num_parser(&str) -> Expr, map!(digit, parse_num));

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;

    #[test]
    fn it_parses_numbers() {
        let (_rem, parsed) = num_parser("1234").unwrap();
        assert_eq!(parsed, ENum(1234));
    }
}

pub fn main() {
    println!("Hello, world!");
}
