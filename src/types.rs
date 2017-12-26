use std::collections::HashMap;

pub type MyResult = Result<f32, Error>;

#[derive(Debug)]
pub enum Error {
    UndefinedVariable(String),
    UndefinedFunction(String),
    UnknownError,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    pub params: Vec<String>,
    pub body: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpr {
    pub condition: (Expr, Expr),
    pub body: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnvValue {
    ComputedResult(f32),
    LambdaRef(Lambda),
    NativeFn(fn(f32) -> f32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    ENum(f32),
    EVar(String),
    EAdd(Box<Expr>, Box<Expr>),
    ESub(Box<Expr>, Box<Expr>),
    EMul(Box<Expr>, Box<Expr>),
    EDiv(Box<Expr>, Box<Expr>),
    EExp(Box<Expr>, Box<Expr>),
    ELet(String, Box<Expr>),
    EIf(Box<IfExpr>, Vec<IfExpr>, Vec<Expr>),
    EFunCall(String, Vec<Expr>),
    EDefun(String, Lambda),
    EReturn(Box<Expr>),
}

#[derive(Clone)]
pub struct Environment(pub HashMap<String, EnvValue>);

use self::EnvValue::*;

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment(HashMap::new());
        let fun_name = String::from("sqrt");
        env.add(fun_name.clone(), NativeFn(|x| x.sqrt()));
        env
    }
    pub fn get(&self, var_name: String) -> Option<EnvValue> {
        self.0.get(&var_name).map(|e| e.clone())
    }
    pub fn add(&mut self, var_name: String, result: EnvValue) -> &mut Environment {
        &self.0.insert(var_name, result);
        self
    }
}
