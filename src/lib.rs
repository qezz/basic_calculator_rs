#[macro_use]
extern crate nom;

use nom::digit;

#[derive(Debug, PartialEq)]
enum Expr {
    ENum(i32),
    EAdd(Box<Expr>, Box<Expr>),
}

use Expr::*;

fn parse_num(str: &str) -> Expr {
    // forgoing all error handling for now
    let num: i32 = str.trim().parse().unwrap();
    Expr::ENum(num)
}

fn parse_add(expr1: Expr, expr2: Expr) -> Expr {
    Expr::EAdd(Box::new(expr1), Box::new(expr2))
}

fn evaluate(expr: Expr) -> i32 {
    match expr {
        ENum(num) => num,
        EAdd(expr1, expr2) => evaluate(*expr1) + evaluate(*expr2),
    }
}

named!(num_parser(&str) -> Expr, map!(ws!(digit), parse_num));
named!(add_parser(&str) -> Expr,
       do_parse!(
           expr1: num_parser >>
           char!('+') >>
           expr2: num_parser >>
           (parse_add(expr1, expr2))
       ));

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;

    #[test]
    fn it_parses_numbers() {
        let (_rem, parsed) = num_parser("1234").unwrap();
        assert_eq!(parsed, ENum(1234));
    }

    #[test]
    fn it_parses_add_statements() {
        let (_rem, parsed) = add_parser("1 + 2").unwrap();
        assert_eq!(parsed, EAdd(Box::new(ENum(1)), Box::new(ENum(2))));
    }

    #[test]
    fn it_evaluates_add_expression() {
        let expr = EAdd(Box::new(ENum(1)), Box::new(ENum(2)));
        assert_eq!(evaluate(expr), 3);
    }
}

pub fn main() {
    println!("Hello, world!");
}
