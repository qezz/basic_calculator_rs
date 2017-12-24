use nom::{digit, alpha, eol};
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
named!(operation<&str, Expr>, alt!(num | parens | return_statement | map!(varname, parse_evar)));
// A factor is either a single operation or one followed by ^ and another factor
named!(factor<&str, Expr>,
       do_parse!(
           op: operation >>
           rem: many0!(tuple!(char!('^'), factor)) >>
           (parse_expr(op, rem))
       ));
// A term is either a single factor or one followed by a (* or /) and another factor
named!(term<&str, Expr>,
       do_parse!(
           f: factor >>
           rem: many0!(tuple!(alt!(char!('*') | char!('/')), factor)) >>
           (parse_expr(f, rem))
       ));
// A sub-expression is either a single term or one followed by a (+ or -) and another term
named!(subexpr<&str, Expr>,
       do_parse!(
           t: term >>
           rem: many0!(tuple!(alt!(char!('+') | char!('-')), term)) >>
           (parse_expr(t, rem))
       ));
// a variable name is just a series of alphabets. We don't want alphanumeric variable names for now.
named!(pub varname<&str, &str>, ws!(alpha));
// a let expression is the let keyword, followed by a variable name, then an equals sign and finally any expression
named!(let_expr<&str, Expr>,
       do_parse!(
           tag!("let") >>
           var_name: varname >>
           char!('=') >>
           expr: expr >>
           (parse_let(var_name, expr))
       ));
named!(return_statement<&str, Expr>,
       do_parse!(
           tag!("return") >>
           expr: expr >>
           (parse_return(expr))
       ));
named!(fun_body<&str, Vec<Expr>>,
       do_parse!(
           ws!(char!('{')) >>
           opt!(char!('\n')) >>
           exprs: many0!(terminated!(ws!(expr), char!(';'))) >>
           opt!(char!('\n')) >>
           ws!(char!('}')) >>
           (exprs)
       ));
named!(arg_list<&str, Vec<&str>>, delimited!(char!('('), separated_list!(char!(','), varname), char!(')')));
named!(defun<&str, Expr>,
       do_parse!(
           tag!("define") >>
           func_name: varname >>
           arg: arg_list >>
           body: fun_body >>
           (parse_defun(func_name, arg, body))
       ));
// an expression is either a let expression or a sub expression, with the former getting higher priority
named!(pub expr<&str, Expr>, alt!(defun | let_expr | subexpr));

fn parse_return(expr: Expr) -> Expr {
    EReturn(Box::new(expr))
}

fn parse_defun(func_name: &str, arg: Vec<&str>, body: Vec<Expr>) -> Expr {
    EDefun(
        func_name.to_string(),
        arg.into_iter().map(|s| s.to_string()).collect(),
        body,
    )
}

fn parse_evar(var_name: &str) -> Expr {
    EVar(var_name.to_string())
}

fn parse_let(var_name: &str, expr: Expr) -> Expr {
    ELet(var_name.to_string(), Box::new(expr))
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
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

    #[test]
    fn test_parse_let_statement() {
        let (_rem, parsed) = expr("let phi = (20 + 30) - 10").unwrap();
        assert_eq!(
            parsed,
            ELet(
                String::from("phi"),
                Box::new(ESub(
                    Box::new(EAdd(Box::new(ENum(20.0)), Box::new(ENum(30.0)))),
                    Box::new(ENum(10.0)),
                )),
            )
        );
    }

    #[test]
    fn test_parse_variables_in_expressions() {
        let (_rem, parsed) = expr("20 + (30 + phi) - 10").unwrap();
        assert_eq!(
            parsed,
            ESub(
                Box::new(EAdd(
                    Box::new(ENum(20.0)),
                    Box::new(EAdd(
                        Box::new(ENum(30.0)),
                        Box::new(EVar(String::from("phi"))),
                    )),
                )),
                Box::new(ENum(10.0)),
            )
        );
    }

    #[test]
    fn test_parse_return_statements() {
        let (_rem, parsed) = expr("return n * n").unwrap();
        assert_eq!(
            parsed,
            EReturn(Box::new(EMul(
                Box::new(EVar(String::from("n"))),
                Box::new(EVar(String::from("n"))),
            )))
        );
    }

    #[test]
    fn test_parse_simple_function_definitions_with_single_argument() {
        let function_definiton = "define square(n) { return n * n; }";
        let (_rem, parsed) = expr(function_definiton).unwrap();
        assert_eq!(
            parsed,
            EDefun(
                String::from("square"),
                vec![String::from("n")],
                vec![
                    EReturn(Box::new(EMul(
                        Box::new(EVar(String::from("n"))),
                        Box::new(EVar(String::from("n"))),
                    ))),
                ],
            )
        );
    }

    #[test]
    fn test_parse_function_definitions() {
        let function_definiton = "define multiply(m, n) {
            let result = m * n;
            return result;
            }";
        let (_rem, parsed) = expr(function_definiton).unwrap();
        assert_eq!(
            parsed,
            EDefun(
                String::from("multiply"),
                vec![String::from("m"), String::from("n")],
                vec![
                    ELet(
                        String::from("result"),
                        Box::new(EMul(
                            Box::new(EVar(String::from("m"))),
                            Box::new(EVar(String::from("n"))),
                        ))
                    ),
                    EReturn(Box::new(EVar(String::from("result")))),
                ],
            )
        )
    }
}
