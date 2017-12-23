use types::*;
use types::Expr::*;

pub fn evaluate(environment: &mut Environment, expr: Expr) -> (&mut Environment, f32) {
    match expr {
        ENum(num) => (environment, num),
        EAdd(expr1, expr2) => {
            let result = evaluate(environment, *expr1).1 + evaluate(environment, *expr2).1;
            (environment, result)
        }
        ESub(expr1, expr2) => {
            let result = evaluate(environment, *expr1).1 - evaluate(environment, *expr2).1;
            (environment, result)
        }
        EMul(expr1, expr2) => {
            let result = evaluate(environment, *expr1).1 * evaluate(environment, *expr2).1;
            (environment, result)
        }
        EDiv(expr1, expr2) => {
            let result = evaluate(environment, *expr1).1 / evaluate(environment, *expr2).1;
            (environment, result)
        }
        EExp(expr1, expr2) => {
            let result = evaluate(environment, *expr1).1.powf(
                evaluate(environment, *expr2)
                    .1,
            );
            (environment, result)
        }
        ELet(varname, expr) => {
            let (old_env, result) = evaluate(environment, *expr);
            (old_env.add(varname, result), result)
        }
        EVar(_varname) => panic!("Undefined"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_evaluate_add_expression() {
        let expr = EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(&mut Environment::new(), expr).1, 3.0);
    }

    #[test]
    fn test_evaluate_subtraction_expression() {
        let expr = ESub(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(&mut Environment::new(), expr).1, 1.0);
    }

    #[test]
    fn test_evaluate_multiplication_expression() {
        let expr = EMul(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(&mut Environment::new(), expr).1, 6.0);
    }

    #[test]
    fn test_evaluate_division_expression() {
        let expr = EDiv(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(&mut Environment::new(), expr).1, 1.5);
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
        assert_eq!(evaluate(&mut Environment::new(), expr).1, 9.2);
    }

    #[test]
    fn test_evaluate_let_expressions() {
        let expr = ELet(
            String::from("phi"),
            Box::new(EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0)))),
        );
        let mut env = Environment::new();
        let (new_env, result) = evaluate(&mut env, expr);
        assert_eq!(result, 3.0);
        assert_eq!(*new_env.get(String::from("phi")), 3.0);
    }
}
