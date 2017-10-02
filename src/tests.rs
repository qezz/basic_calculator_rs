use types::Expr::*;
use parser::*;
use evaluator::*;

#[test]
fn it_parses_add_statements() {
    let (_rem, parsed) = expr("1 + 2").unwrap();
    assert_eq!(parsed, EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
}

#[test]
fn it_parses_subtraction_statements() {
    let (_rem, parsed) = expr("1 - 2").unwrap();
    assert_eq!(parsed, ESub(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
}

#[test]
fn it_parses_multiplication_statements() {
    let (_rem, parsed) = expr("1 * 2").unwrap();
    assert_eq!(parsed, EMul(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
}

#[test]
fn it_parses_division_statements() {
    let (_rem, parsed) = expr("1 / 2").unwrap();
    assert_eq!(parsed, EDiv(Box::new(ENum(1.0)), Box::new(ENum(2.0))));
}

#[test]
fn it_evaluates_add_expression() {
    let expr = EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0)));
    assert_eq!(evaluate(expr), 3.0);
}

#[test]
fn it_evaluates_subtraction_expression() {
    let expr = ESub(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
    assert_eq!(evaluate(expr), 1.0);
}

#[test]
fn it_evaluates_multiplication_expression() {
    let expr = EMul(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
    assert_eq!(evaluate(expr), 6.0);
}

#[test]
fn it_evaluates_division_expression() {
    let expr = EDiv(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
    assert_eq!(evaluate(expr), 1.5);
}
