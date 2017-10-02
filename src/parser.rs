use nom::digit;
use types::Expr;
use types::Expr::*;
use std::str::FromStr;

// Use the classic solution to break left recursion in a LL(1) recursive descent parser
// Solution can be found here: https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm#classic

// Parse numbers as floats
named!(num<&str, Expr>, map!(ws!(digit),  parse_num));
// Parse an expression with parantheses
named!(parens<&str, Expr>, delimited!(ws!(char!('(')), expr, ws!(char!(')'))));
// An operation is either a number or a parantesised expression
named!(operation<&str, Expr>, alt!(num | parens));
// A factor is either a single operation or one followed by ^ and another factor
named!(factor<&str, Expr>,
       do_parse!(
           op: operation >>
           rem: many0!(tuple!(char!('^'), factor)) >>
           (parse_factor(op, rem))
       ));
// A term is either a single factor or one followed by a (* or /) and another factor
named!(term<&str, Expr>,
       do_parse!(
           f: factor >>
           rem: many0!(tuple!(alt!(char!('*') | char!('/')), factor)) >>
           (parse_term(f, rem))
       ));
// A expression is either a single term or one followed by a (+ or -) and another term
named!(pub expr(&str) -> Expr,
       do_parse!(
           t: term >>
           rem: many0!(tuple!(alt!(char!('+') | char!('-')), term)) >>
           (parse_expr(t, rem))
       ));

// Let the duplication in below 3 functions remain for now, it will help with debugging
// Can be removed as the last step, once we know everything is working fine
fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    println!("inside parse expression, remaining is {:?}", rem);
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_factor(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    println!("inside parse factor, remaining is {:?}", rem);
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_term(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    println!("inside parse term, remaining is {:?}", rem);
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (char, Expr), expr1: Expr) -> Expr {
    let (op, expr2) = tup;
    match op {
        '+' => EAdd(Box::new(expr1), Box::new(expr2)),
        '-' => ESub(Box::new(expr1), Box::new(expr2)),
        '*' => EMul(Box::new(expr1), Box::new(expr2)),
        '/' => EDiv(Box::new(expr1), Box::new(expr2)),
        '^' => EExp(Box::new(expr1), Box::new(expr2)),
        _ => panic!("Unknown Operation"),
    }
}

fn parse_num(num: &str) -> Expr {
    println!("inside parse num");
    // forgoing all error handling for now
    ENum(f32::from_str(num).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_add_statement() {
        let (_rem, parsed) = expr("1 + 2").unwrap();
        assert_eq!(parsed, EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
    }

    #[test]
    fn test_parse_subtraction_statement() {
        let (_rem, parsed) = expr("1 - 2").unwrap();
        assert_eq!(parsed, ESub(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
    }

    #[test]
    fn test_parse_multiplication_statement() {
        let (_rem, parsed) = expr("1 * 2").unwrap();
        assert_eq!(parsed, EMul(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
    }

    #[test]
    fn test_parse_multi_level_expression() {
        let (_rem, parsed) = expr("1 * 2 + 3 / 4 ^ 6").unwrap();
        let expected = EAdd(
            Box::new(EMul(Box::new(ENum(1.0)), Box::new(ENum(2.0)))),
            Box::new(EDiv(
                Box::new(ENum(3.0)),
                Box::new(EExp(Box::new(ENum(4.0)), Box::new(ENum(6.0)))),
            )),
        );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_expression_with_parantheses() {
        let (_rem, parsed) = expr("(1 + 2) * 3").unwrap();
        let expected = EMul(
            Box::new(EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0)))),
            Box::new(ENum(3.0)),
        );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_division_statement() {
        let (_rem, parsed) = expr("1 / 2").unwrap();
        assert_eq!(parsed, EDiv(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
    }
}
