use types::Expr::*;
use parser::*;
use evaluator::*;

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
