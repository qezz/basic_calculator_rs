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
        EFunCall(func_name, args) => {
            let defun = environment.get(func_name.clone());
            match defun {
                EDefun(_, params, body) => {
                    let mut cloned_environment = environment.clone();
                    params.into_iter().zip(args.into_iter()).fold(
                        &mut cloned_environment,
                        |env, value| env.add(value.0, value.1),
                    );
                    let result = body.into_iter().fold(
                        (&mut cloned_environment, 0.0),
                        |env, expr| evaluate(env.0, expr),
                    );
                    (environment, result.1)
                }
                ENative(f) => {
                    //Always assuming presence of a single f32 argument. Need better error handling.
                    let mut new_env = environment.clone();
                    let (_, result) = evaluate(&mut new_env, args.into_iter().nth(0).unwrap());
                    (environment, f(result))
                }
                _ => panic!("Undefined function {}", func_name),
            }
        }
        ENative(_) => panic!("Runtime error. Native expressions shouldn't be evaluated directly."),
        EReturn(expr) => evaluate(environment, *expr),
        EIf(ifexpr, elseifexprs, elsebody) => {
            let (lhs, rhs) = ifexpr.clone().condition;
            let (_, lhsresult) = evaluate(environment, lhs);
            let (_, rhsresult) = evaluate(environment, rhs);
            if lhsresult == rhsresult {
                let mut cloned_environment = environment.clone();
                let result = ifexpr.body.into_iter().fold(
                    (&mut cloned_environment, 0.0),
                    |env, expr| evaluate(env.0, expr),
                );
                (environment, result.1)
            } else {
                let mut cloned_environment = environment.clone();
                let maybe_else_if_result = elseifexprs
                    .into_iter()
                    .map(|ifexpr| {
                        let (lhs, rhs) = ifexpr.clone().condition;
                        let (_, lhsresult) = evaluate(environment, lhs);
                        let (_, rhsresult) = evaluate(environment, rhs);
                        (lhsresult == rhsresult, ifexpr.body)
                    })
                    .find(|pair| pair.0 == true)
                    .map(|p| {
                        let result = p.1.into_iter().fold(
                            (&mut cloned_environment, 0.0),
                            |env, expr| evaluate(env.0, expr),
                        );
                        result.1
                    });
                let result = maybe_else_if_result.unwrap_or_else(|| {
                    let result = elsebody.into_iter().fold(
                        (&mut cloned_environment, 0.0),
                        |env, expr| evaluate(env.0, expr),
                    );
                    result.1
                });
                (environment, result)
            }
        }
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

    #[test]
    fn test_evaluate_function_application() {
        let fun_name = String::from("multiply");
        let fun_expr = EDefun(
            fun_name.clone(),
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
        );
        let mut env = Environment::new();
        env.add(fun_name.clone(), fun_expr.clone());

        let first_arg_expr = EMul(Box::new(ENum(2.0)), Box::new(ENum(3.0)));
        let fun_call_expr = EFunCall(fun_name.clone(), vec![first_arg_expr, ENum(4.0)]);

        let (_new_env, result) = evaluate(&mut env, fun_call_expr);
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_evaluate_native_function_calls() {
        let fun_call_expr = EFunCall(
            String::from("sqrt"),
            vec![EMul(Box::new(ENum(3.0)), Box::new(ENum(3.0)))],
        );
        let mut env = Environment::new();

        let (_new_env, result) = evaluate(&mut env, fun_call_expr);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_evaluate_simple_if_statements_when_condition_is_true() {
        let if_expr = EIf(
            Box::new(IfExpr {
                condition: (EVar(String::from("n")), ENum(1.0)),
                body: vec![EReturn(Box::new(ENum(1.0)))],
            }),
            vec![],
            vec![EReturn(Box::new(ENum(2.0)))],
        );
        let mut env = Environment::new();
        env.add(String::from("n"), ENum(1.0));

        let (_new_env, result) = evaluate(&mut env, if_expr);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_evaluate_simple_if_statements_when_condition_is_false() {
        let if_expr = EIf(
            Box::new(IfExpr {
                condition: (EVar(String::from("n")), ENum(2.0)),
                body: vec![EReturn(Box::new(ENum(1.0)))],
            }),
            vec![],
            vec![EReturn(Box::new(ENum(2.0)))],
        );
        let mut env = Environment::new();
        env.add(String::from("n"), ENum(1.0));

        let (_new_env, result) = evaluate(&mut env, if_expr);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_evaluate_simple_if_else_if_statements_when_if_condition_is_false() {
        let if_statement = IfExpr {
            condition: (EVar(String::from("n")), ENum(1.0)),
            body: vec![EReturn(Box::new(ENum(1.0)))],
        };
        let first_else_if = IfExpr {
            condition: (EVar(String::from("n")), ENum(2.0)),
            body: vec![
                ELet(String::from("x"), Box::new(ENum(3.0))),
                EReturn(Box::new(EVar(String::from("x")))),
            ],
        };
        let second_else_if = IfExpr {
            condition: (EVar(String::from("n")), ENum(3.0)),
            body: vec![
                ELet(String::from("y"), Box::new(ENum(4.0))),
                EReturn(Box::new(EMul(
                    Box::new(EVar(String::from("y"))),
                    Box::new(EVar(String::from("y"))),
                ))),
            ],
        };
        let if_expr = EIf(
            Box::new(if_statement),
            vec![first_else_if, second_else_if],
            vec![EReturn(Box::new(ENum(2.0)))],
        );
        let mut env = Environment::new();
        env.add(String::from("n"), ENum(3.0));

        let (_new_env, result) = evaluate(&mut env, if_expr);
        assert_eq!(result, 16.0);
    }
}
