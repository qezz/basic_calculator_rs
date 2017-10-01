use nom::digit;
use types::Expr;

named!(pub num_parser(&str) -> Expr, map!(ws!(digit), parse_num));
named!(pub add_parser(&str) -> Expr,
       do_parse!(
           expr1: num_parser >>
           char!('+') >>
           expr2: num_parser >>
           (parse_add(expr1, expr2))
       ));

fn parse_num(str: &str) -> Expr {
    // forgoing all error handling for now
    let num: i32 = str.trim().parse().unwrap();
    Expr::ENum(num)
}

fn parse_add(expr1: Expr, expr2: Expr) -> Expr {
    Expr::EAdd(Box::new(expr1), Box::new(expr2))
}
