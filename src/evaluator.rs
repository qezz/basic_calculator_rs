use types::*;
use types::Error::*;
use types::Expr::*;
use types::EnvValue::*;
use types::Result;
use std::result::Result as StdResult;

pub fn evaluate(env: &mut Environment, expr: Expr) -> Result {
    match expr {
        ENum(num) => Ok(num),
        EAdd(expr1, expr2) => Ok(evaluate(env, *expr1)? + evaluate(env, *expr2)?),
        ESub(expr1, expr2) => Ok(evaluate(env, *expr1)? - evaluate(env, *expr2)?),
        EMul(expr1, expr2) => Ok(evaluate(env, *expr1)? * evaluate(env, *expr2)?),
        EDiv(expr1, expr2) => Ok(evaluate(env, *expr1)? / evaluate(env, *expr2)?),
        EExp(expr1, expr2) => Ok(evaluate(env, *expr1)?.powf(evaluate(env, *expr2)?)),
        ELet(varname, expr) => {
            let result = evaluate(env, *expr.clone())?;
            env.add(varname, ComputedResult(result));
            Ok(result)
        }
        EVar(varname) => {
            if let Some(result) = env.get(varname.clone()) {
                match result {
                    ComputedResult(v) => Ok(v),
                    _ => Err(InvalidVariableReference(varname.clone())),
                }
            } else {
                Err(UndefinedVariable(varname.clone()))
            }
        }
        EDefun(fun_name, Lambda { params, body }) => {
            env.add(fun_name.clone(), LambdaRef(Lambda { params, body }));
            Ok(0.0)
        }
        EFunCall(func_name, args) => {
            if let Some(defun) = env.get(func_name.clone()) {
                match defun {
                    LambdaRef(Lambda { params, body }) => {
                        if args.len() != params.len() {
                            Err(InvalidLambdaArgs(
                                func_name.clone(),
                                params.len(),
                                args.len(),
                            ))
                        } else {
                            let mut cloned_environment = env.clone();
                            let maybe_args: StdResult<Vec<f32>, _> =
                                args.into_iter().map(|arg| evaluate(env, arg)).collect();
                            params.into_iter().zip(maybe_args?.into_iter()).fold(
                                &mut cloned_environment,
                                |env, value| env.add(value.0, ComputedResult(value.1)),
                            );
                            body.into_iter().fold(Ok(0.0), |_, expr| {
                                evaluate(&mut cloned_environment, expr)
                            })
                        }
                    }
                    NativeFn(f) => {
                        if args.len() > 1 {
                            Err(InvalidNativeFunctionArgs(func_name.clone(), args.len()))
                        } else {
                            let result = evaluate(env, args.into_iter().nth(0).unwrap())?;
                            Ok(f(result))
                        }
                    }
                    _ => Err(InvalidFunctionReference(func_name.clone())),
                }
            } else {
                Err(UndefinedFunction(func_name.clone()))
            }
        }
        EReturn(expr) => evaluate(env, *expr),
        EIf(ifexprs, elseexpr) => {
            let bools: StdResult<Vec<bool>, _> = ifexprs
                .iter()
                .map(|ifexpr| {
                    let (lhs, rhs) = ifexpr.clone().condition;
                    Ok(evaluate(env, lhs)? == evaluate(env, rhs)?)
                })
                .collect();
            ifexprs
                .into_iter()
                .zip(bools?.into_iter())
                .find(|p| p.1 == true)
                .map(|(ex, _)| {
                    let mut cloned_environment = env.clone();
                    let result = ex.body.into_iter().fold(Ok(0.0), |_, expr| {
                        evaluate(&mut cloned_environment, expr)
                    });
                    result
                })
                .unwrap_or_else(|| {
                    let mut cloned_environment = env.clone();
                    elseexpr.into_iter().fold(
                        Ok(0.0),
                        |_env, expr| evaluate(&mut cloned_environment, expr),
                    )
                })
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_evaluate_add_expression() {
        let expr = EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(&mut Environment::new(), expr).unwrap(), 3.0);
    }

    #[test]
    fn test_evaluate_subtraction_expression() {
        let expr = ESub(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(&mut Environment::new(), expr).unwrap(), 1.0);
    }

    #[test]
    fn test_evaluate_multiplication_expression() {
        let expr = EMul(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(&mut Environment::new(), expr).unwrap(), 6.0);
    }

    #[test]
    fn test_evaluate_division_expression() {
        let expr = EDiv(Box::new(ENum(3.0)), Box::new(ENum(2.0)));
        assert_eq!(evaluate(&mut Environment::new(), expr).unwrap(), 1.5);
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
        assert_eq!(evaluate(&mut Environment::new(), expr).unwrap(), 9.2);
    }

    #[test]
    fn test_evaluate_let_expressions() {
        let var_name = String::from("phi");
        let let_expr = EAdd(Box::new(ENum(1.0)), Box::new(ENum(2.0)));
        let expr = ELet(var_name.clone(), Box::new(let_expr.clone()));
        let mut env = Environment::new();
        assert_eq!(evaluate(&mut env, expr.clone()).unwrap(), 3.0);
        assert_eq!(env.get(var_name.clone()), Some(ComputedResult(3.0)));
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
        env.add(var_name.clone(), ComputedResult(20.0));
        assert_eq!(evaluate(&mut env, expr).unwrap(), 60.0);
    }

    #[test]
    fn test_evaluate_simple_return_statements() {
        let expr = EReturn(Box::new(EMul(Box::new(ENum(3.0)), Box::new(ENum(2.0)))));
        let mut env = Environment::new();
        assert_eq!(evaluate(&mut env, expr).unwrap(), 6.0);
    }

    #[test]
    fn test_evaluate_return_statements_that_use_environment() {
        let var_name = String::from("phi");
        let expr = EReturn(Box::new(
            EMul(Box::new(ENum(3.0)), Box::new(EVar(var_name.clone()))),
        ));
        let mut env = Environment::new();
        env.add(var_name.clone(), ComputedResult(2.0));
        assert_eq!(evaluate(&mut env, expr).unwrap(), 6.0);
    }

    #[test]
    fn test_evaluate_function_definitions() {
        let lambda = Lambda {
            params: vec![String::from("n")],
            body: vec![
                EReturn(Box::new(EMul(
                    Box::new(EVar(String::from("n"))),
                    Box::new(EVar(String::from("n"))),
                ))),
            ],
        };
        let expr = EDefun(String::from("square"), lambda.clone());
        let mut env = Environment::new();
        let result = evaluate(&mut env, expr).unwrap();
        assert_eq!(
            env.get(String::from("square")),
            Some(LambdaRef(lambda.clone()))
        );
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_evaluate_function_application() {
        let fun_name = String::from("multiply");
        let lambda = Lambda {
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
        };

        let mut env = Environment::new();
        env.add(fun_name.clone(), LambdaRef(lambda));

        let first_arg_expr = EMul(Box::new(ENum(2.0)), Box::new(ENum(3.0)));
        let fun_call_expr = EFunCall(fun_name.clone(), vec![first_arg_expr, ENum(4.0)]);

        assert_eq!(evaluate(&mut env, fun_call_expr).unwrap(), 24.0);
    }

    #[test]
    fn test_evaluate_native_function_calls() {
        let fun_call_expr = EFunCall(
            String::from("sqrt"),
            vec![EMul(Box::new(ENum(3.0)), Box::new(ENum(3.0)))],
        );
        let mut env = Environment::new();

        assert_eq!(evaluate(&mut env, fun_call_expr).unwrap(), 3.0);
    }

    #[test]
    fn test_evaluate_simple_if_statements_when_condition_is_true() {
        let if_expr = EIf(
            vec![
                IfExpr {
                    condition: (EVar(String::from("n")), ENum(1.0)),
                    body: vec![EReturn(Box::new(ENum(1.0)))],
                },
            ],
            vec![EReturn(Box::new(ENum(2.0)))],
        );
        let mut env = Environment::new();
        env.add(String::from("n"), ComputedResult(1.0));

        assert_eq!(evaluate(&mut env, if_expr).unwrap(), 1.0);
    }

    #[test]
    fn test_evaluate_simple_if_statements_when_condition_is_false() {
        let if_expr = EIf(
            vec![
                IfExpr {
                    condition: (EVar(String::from("n")), ENum(2.0)),
                    body: vec![EReturn(Box::new(ENum(1.0)))],
                },
            ],
            vec![EReturn(Box::new(ENum(2.0)))],
        );
        let mut env = Environment::new();
        env.add(String::from("n"), ComputedResult(1.0));

        assert_eq!(evaluate(&mut env, if_expr).unwrap(), 2.0);
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
            vec![if_statement, first_else_if, second_else_if],
            vec![EReturn(Box::new(ENum(2.0)))],
        );
        let mut env = Environment::new();
        env.add(String::from("n"), ComputedResult(3.0));

        assert_eq!(evaluate(&mut env, if_expr).unwrap(), 16.0);
    }

    #[test]
    fn test_evaluate_recursive_function_calls() {
        let fun_name = String::from("fibrecursive");
        let recursive_function = Lambda {
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
                            Box::new(EFunCall(
                                fun_name.clone(),
                                vec![
                                    ESub(
                                        Box::new(EVar(String::from("n"))),
                                        Box::new(ENum(2.0))
                                    ),
                                ],
                            )),
                        ))),
                    ]
                ),
            ],
        };
        let mut env = Environment::new();
        env.add(fun_name.clone(), LambdaRef(recursive_function));

        let fun_call_expr = EFunCall(fun_name.clone(), vec![ENum(4.0)]);

        assert_eq!(evaluate(&mut env, fun_call_expr).unwrap(), 3.0);
    }
}
