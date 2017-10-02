use types::Expr;
use types::Expr::*;

pub fn evaluate(expr: Expr) -> f32 {
    match expr {
        ENum(num) => num,
        EAdd(expr1, expr2) => evaluate(*expr1) + evaluate(*expr2),
        ESub(expr1, expr2) => evaluate(*expr1) - evaluate(*expr2),
        EMul(expr1, expr2) => evaluate(*expr1) * evaluate(*expr2),
        EDiv(expr1, expr2) => evaluate(*expr1) / evaluate(*expr2),
        EExp(expr1, expr2) => evaluate(*expr1).powf(evaluate(*expr2)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_evaluate_add_expression() {
        let expr = EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(expr), 3.0);
    }

    #[test]
    fn test_evaluate_subtraction_expression() {
        let expr = ESub(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(expr), 1.0);
    }

    #[test]
    fn test_evaluate_multiplication_expression() {
        let expr = EMul(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(expr), 6.0);
    }

    #[test]
    fn test_evaluate_division_expression() {
        let expr = EDiv(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(expr), 1.5);
    }

    #[test]
    fn test_evaluate_nested_arithmetic_expression() {
        let expr = EAdd(
            Box::new(EMul(Box::new(ENum(1.0)), Box::new(ENum(2.0)))),
            Box::new(EDiv(
                Box::new(EExp(Box::new(ENum(6.0)), Box::new(ENum(2.0)))),
                Box::new(ENum(5.0)),
            )),
        );
        assert_eq!(evaluate(expr), 9.2);
    }
}
