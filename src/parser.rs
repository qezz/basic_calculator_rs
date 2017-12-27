use nom::{digit, alpha};
use nom::IResult::*;
use types::Lambda;
use types::Error;
use types::Error::ParseError;
use types::Expr;
use types::IfExpr;
use types::Expr::*;
use std::str::FromStr;

// Use the classic solution to break left recursion in a LL(1) recursive descent parser
// Solution can be found here: https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm#classic

named!(num<&str, Expr>, map!(ws!(digit),  parse_num));
named!(parens<&str, Expr>, ws!(delimited!(char!('('), mathexpr, char!(')'))));
named!(operation<&str, Expr>, alt!( complete!(funcall) | map!(varname, parse_evar) | num | parens));
named!(factor<&str, Expr>,
       do_parse!(
           op: operation >>
           rem: many0!(tuple!(char!('^'), factor)) >>
           (parse_expr(op, rem))
       ));
named!(term<&str, Expr>,
       do_parse!(
           f: factor >>
           rem: many0!(tuple!(alt!(char!('*') | char!('/')), factor)) >>
           (parse_expr(f, rem))
       ));
named!(mathexpr<&str, Expr>,
       do_parse!(
           t: term >>
           rem: many0!(tuple!(alt!(char!('+') | char!('-')), term)) >>
           (parse_expr(t, rem))
       ));
named!(varname<&str, &str>, ws!(alpha));
named!(let_expr<&str, Expr>,
       do_parse!(
           tag!("let") >>
           var_name: varname >>
           char!('=') >>
           expr: mathexpr >>
           (parse_let(var_name, expr))
       ));
named!(return_statement<&str, Expr>,
       do_parse!(
           tag!("return") >>
           expr: mathexpr >>
           (parse_return(expr))
       ));
named!(block<&str, Vec<Expr>>,
       do_parse!(
           ws!(char!('{')) >>
           opt!(char!('\n')) >>
           exprs: many0!(terminated!(ws!(nested_expr), char!(';'))) >>
           opt!(char!('\n')) >>
           ws!(char!('}')) >>
           (exprs)
       ));
named!(arg_list<&str, Vec<&str>>, delimited!(char!('('), separated_list!(char!(','), varname), char!(')')));
named!(defun<&str, Expr>,
       do_parse!(
           tag!("define") >>
           func_name: varname >>
           params: arg_list >>
           body: block >>
           (parse_defun(func_name, params, body))
       ));
named!(funcall<&str, Expr>,
       do_parse!(
           func_name: varname >>
           args: ws!(delimited!(char!('('), separated_list!(char!(','), expr), char!(')'))) >>
           (parse_funcall(func_name, args))
       ));
named!(if_cond<&str, (Expr, Expr)>, delimited!(char!('('), separated_pair!(expr, ws!(tag!("==")), expr), char!(')')));
named!(single_if<&str, IfExpr>,
       do_parse!(
           ws!(tag!("if")) >>
           cond: if_cond >>
           body: block >>
           (parse_single_if(cond, body))
       ));
named!(ifexpr<&str, Expr>,
       do_parse!(
           ifexpr: single_if >>
           else_ifs: many0!(do_parse!(tag!("else") >> ifexpr: single_if >> (ifexpr))) >>
           elseexpr: do_parse!(tag!("else") >> body: block >> (body)) >>
           (parse_if_expression(ifexpr, else_ifs, elseexpr))
       ));
named!(nested_expr<&str, Expr>, alt!(let_expr | ifexpr | return_statement | mathexpr));
named!(pub expr<&str, Expr>, alt!(defun | nested_expr));

pub fn parse(input: &str) -> Result<Expr, Error> {
    match expr(input) {
        Done(_, expr) => Ok(expr),
        _ => Err(ParseError),
    }
}

fn parse_if_expression(ifexpr: IfExpr, else_ifs: Vec<IfExpr>, elseexpr: Vec<Expr>) -> Expr {
    let mut ifs = vec![ifexpr];
    ifs.extend(else_ifs);
    EIf(ifs, elseexpr)
}

fn parse_single_if(condition: (Expr, Expr), body: Vec<Expr>) -> IfExpr {
    IfExpr { condition, body }
}

fn parse_funcall(name: &str, args: Vec<Expr>) -> Expr {
    EFunCall(name.to_string(), args)
}

fn parse_return(expr: Expr) -> Expr {
    EReturn(Box::new(expr))
}

fn parse_defun(func_name: &str, params: Vec<&str>, body: Vec<Expr>) -> Expr {
    EDefun(
        func_name.to_string(),
        Lambda {
            params: params.into_iter().map(|s| s.to_string()).collect(),
            body,
        },
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
        let parsed = parse("1 + 2").unwrap();
        assert_eq!(parsed, EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
    }

    #[test]
    fn test_parse_subtraction_statement() {
        let parsed = parse("1 - 2").unwrap();
        assert_eq!(parsed, ESub(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
    }

    #[test]
    fn test_parse_multiplication_statement() {
        let parsed = parse("1 * 2").unwrap();
        assert_eq!(parsed, EMul(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
    }

    #[test]
    fn test_parse_multi_level_expression() {
        let parsed = parse("1 * 2 + 3 / 4 ^ 6").unwrap();
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
        let parsed = parse("(1 + 2) * 3").unwrap();
        let expected = EMul(
            Box::new(EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0)))),
            Box::new(ENum(3.0)),
        );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_division_statement() {
        let parsed = parse("1 / 2").unwrap();
        assert_eq!(parsed, EDiv(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
    }

    #[test]
    fn test_parse_let_statement() {
        let parsed = parse("let phi = (20 + 30) - 10").unwrap();
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
        let parsed = parse("20 + (30 + phi) - 10").unwrap();
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
        let parsed = parse("return n * n").unwrap();
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
        let parsed = parse(function_definiton).unwrap();
        assert_eq!(
            parsed,
            EDefun(
                String::from("square"),
                Lambda {
                    params: vec![String::from("n")],
                    body: vec![
                        EReturn(Box::new(EMul(
                            Box::new(EVar(String::from("n"))),
                            Box::new(EVar(String::from("n"))),
                        ))),
                    ],
                },
            )
        );
    }

    #[test]
    fn test_parse_function_definitions() {
        let function_definiton = "define multiply(m, n) {
            let result = m * n;
            return result;
            }";
        let parsed = parse(function_definiton).unwrap();
        assert_eq!(
            parsed,
            EDefun(
                String::from("multiply"),
                Lambda {
                    params: vec![String::from("m"), String::from("n")],
                    body: vec![
                        ELet(
                            String::from("result"),
                            Box::new(EMul(
                                Box::new(EVar(String::from("m"))),
                                Box::new(EVar(String::from("n"))),
                            ))
                        ),
                        EReturn(Box::new(EVar(String::from("result")))),
                    ],
                },
            )
        )
    }

    #[test]
    fn test_parse_function_application() {
        let function_call = "multiply(5, 6)";
        let parsed = parse(function_call).unwrap();
        assert_eq!(
            parsed,
            EFunCall(String::from("multiply"), vec![ENum(5.0), ENum(6.0)])
        );
    }

    #[test]
    fn test_parses_simple_if_else_statement() {
        let if_definition = "if (n == 1) {
             return 1;
            } else {
             return 2;
            }";
        let parsed = parse(if_definition).unwrap();
        assert_eq!(
            parsed,
            EIf(
                vec![
                    IfExpr {
                        condition: (EVar(String::from("n")), ENum(1.0)),
                        body: vec![EReturn(Box::new(ENum(1.0)))],
                    },
                ],
                vec![EReturn(Box::new(ENum(2.0)))],
            )
        );
    }

    #[test]
    fn test_parses_if_else_if_statement() {
        let if_definition = "if (n == 1) {
             return 1;
            } else if(n == 2) {
             let x = 3;
             return x;
            } else if(n==3) {
             let y = 4;
             return y * y;
            } else {
             return 2;
            }";
        let parsed = parse(if_definition).unwrap();
        assert_eq!(
            parsed,
            EIf(
                vec![
                    IfExpr {
                        condition: (EVar(String::from("n")), ENum(1.0)),
                        body: vec![EReturn(Box::new(ENum(1.0)))],
                    },
                    IfExpr {
                        condition: (EVar(String::from("n")), ENum(2.0)),
                        body: vec![
                            ELet(String::from("x"), Box::new(ENum(3.0))),
                            EReturn(Box::new(EVar(String::from("x")))),
                        ],
                    },
                    IfExpr {
                        condition: (EVar(String::from("n")), ENum(3.0)),
                        body: vec![
                            ELet(String::from("y"), Box::new(ENum(4.0))),
                            EReturn(Box::new(EMul(
                                Box::new(EVar(String::from("y"))),
                                Box::new(EVar(String::from("y"))),
                            ))),
                        ],
                    },
                ],
                vec![EReturn(Box::new(ENum(2.0)))],
            )
        );
    }

    #[test]
    fn test_parses_recursive_function_definitions() {
        let recursive_function = "define fibrecursive(n) {
            if (n == 1) {
              return 1;
            } else if (n == 2) {
              return 1;
            } else {
              return fibrecursive(n - 1) + fibrecursive(n);
            };
          }";
        let parsed = parse(recursive_function).unwrap();
        let fun_name = String::from("fibrecursive");
        assert_eq!(
            parsed,
            EDefun(
                fun_name.clone(),
                Lambda {
                    params: vec![String::from("n")],
                    body: vec![
                        EIf(
                            vec![
                                IfExpr {
                                    condition: (EVar(String::from("n")), ENum(1.0)),
                                    body: vec![EReturn(Box::new(ENum(1.0)))],
                                },
                                IfExpr {
                                    condition: (EVar(String::from("n")), ENum(2.0)),
                                    body: vec![EReturn(Box::new(ENum(1.0)))],
                                },
                            ],
                            vec![
                                EReturn(Box::new(EAdd(
                                    Box::new(EFunCall(
                                        fun_name.clone(),
                                        vec![
                                            ESub(
                                                Box::new(EVar(String::from("n"))),
                                                Box::new(ENum(1.0))
                                            ),
                                        ],
                                    )),
                                    Box::new(
                                        EFunCall(fun_name.clone(), vec![EVar(String::from("n"))]),
                                    ),
                                ))),
                            ]
                        ),
                    ],
                },
            )
        );
    }
}
