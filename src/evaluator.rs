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
            let (old_env, result) = evaluate(environment, *expr.clone());
            (old_env.add(varname, *expr.clone()), result)
        }
        EVar(varname) => {
            let result = environment.get(varname);
            evaluate(environment, result)
        }
        EDefun(fun_name, params, body) => {
            (
                environment.add(fun_name.clone(), EDefun(fun_name.clone(), params, body)),
                0.0,
            )
        }
        EFunCall(_, _) => (environment, 0.0),
        EReturn(expr) => evaluate(environment, *expr),
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
        let var_name = String::from("phi");
        let let_expr = EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0)));
        let expr = ELet(var_name.clone(), Box::new(let_expr.clone()));
        let mut env = Environment::new();
        let (new_env, result) = evaluate(&mut env, expr.clone());
        assert_eq!(result, 3.0);
        assert_eq!(new_env.get(var_name.clone()), let_expr.clone());
    }

    #[test]
    fn test_evaluate_expressions_with_variables() {
        let var_name = String::from("phi");
        let expr = ESub(
            Box::new(EAdd(
                Box::new(ENum(20.0)),
                Box::new(
                    EAdd(Box::new(ENum(30.0)), Box::new(EVar(var_name.clone()))),
                ),
            )),
            Box::new(ENum(10.0)),
        );
        let mut env = Environment::new();
        env.add(var_name.clone(), ENum(20.0));
        let (_new_env, result) = evaluate(&mut env, expr);
        assert_eq!(result, 60.0);
    }

    #[test]
    fn test_evaluate_simple_return_statements() {
        let expr = EReturn(Box::new(EMul(Box::new(ENum(3.0)), Box::new(ENum(2.0)))));
        let mut env = Environment::new();
        let (_new_env, result) = evaluate(&mut env, expr);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_evaluate_return_statements_that_use_environment() {
        let var_name = String::from("phi");
        let expr = EReturn(Box::new(
            EMul(Box::new(ENum(3.0)), Box::new(EVar(var_name.clone()))),
        ));
        let mut env = Environment::new();
        env.add(var_name.clone(), ENum(2.0));
        let (_new_env, result) = evaluate(&mut env, expr);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_evaluate_function_definitions() {
        let expr = EDefun(
            String::from("square"),
            vec![String::from("n")],
            vec![
                EReturn(Box::new(EMul(
                    Box::new(EVar(String::from("n"))),
                    Box::new(EVar(String::from("n"))),
                ))),
            ],
        );
        let mut env = Environment::new();
        let (new_env, result) = evaluate(&mut env, expr.clone());
        assert_eq!(new_env.get(String::from("square")), expr.clone());
        assert_eq!(result, 0.0);
    }
}
