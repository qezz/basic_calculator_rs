use nom::digit;
use types::Expr;
use types::Expr::*;

named!(num(&str) -> Expr, map!(ws!(digit), parse_num));
named!(pub expr(&str) -> Expr,
       do_parse!(
           expr1: num >>
           op: alt!(char!('+') | char!('-') | char!('*') | char!('/')) >>
           expr2: num >>
           (parse_simple_expr(op, expr1, expr2))
       ));

fn parse_num(str: &str) -> Expr {
    // forgoing all error handling for now
    let num: f32 = str.trim().parse().unwrap();
    ENum(num)
}

fn parse_simple_expr(op: char, expr1: Expr, expr2: Expr) -> Expr {
    match op {
        '+' => EAdd(Box::new(expr1), Box::new(expr2)),
        '-' => ESub(Box::new(expr1), Box::new(expr2)),
        '*' => EMul(Box::new(expr1), Box::new(expr2)),
        '/' => EDiv(Box::new(expr1), Box::new(expr2)),
        _ => panic!("Unknown operation"),
    }
}
