use std::collections::HashMap;
use std::fmt;
use std::result;

pub type Result = result::Result<f32, Error>;

#[derive(Debug)]
pub enum Error {
    UndefinedVariable(String),
    InvalidVariableReference(String),
    InvalidFunctionReference(String),
    InvalidLambdaArgs(String, usize, usize),
    InvalidNativeFunctionArgs(String, usize),
    UndefinedFunction(String),
    ParseError,
}

use types::Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UndefinedVariable(ref varname) => write!(f, "Undefined Variable: {}", varname),
            InvalidVariableReference(ref varname) => {
                write!(
                    f,
                    "Syntax Error: Variable {} doesn't refer to a computed value",
                    varname
                )
            }
            InvalidFunctionReference(ref fun_name) => {
                write!(
                    f,
                    "Syntax Error: Function {} doesn't refer to a lambda",
                    fun_name
                )
            }
            InvalidLambdaArgs(ref fun_name, ref expected, ref actual) => {
                write!(
                    f,
                    "Syntax Error: Function '{}' expects only {} arguments, but got {}",
                    fun_name,
                    expected,
                    actual
                )
            }
            InvalidNativeFunctionArgs(ref native_fn_name, ref actual) => {
                write!(
                    f,
                    "Syntax Error: Native function '{}' can only be called with a single argument, but got {}",
                    native_fn_name,
                    actual
                )
            }
            UndefinedFunction(ref varname) => write!(f, "Undefined Function: {}", varname),
            ParseError => write!(f, "Unable the parse the input. Please recheck."),
        }
    }
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
    EIf(Vec<IfExpr>, Vec<Expr>),
    EFunCall(String, Vec<Expr>),
    EDefun(String, Lambda),
    EReturn(Box<Expr>),
}

#[derive(Clone)]
pub struct Environment(HashMap<String, EnvValue>);

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

pub fn display(r: Result) -> String {
    match r {
        Ok(value) => value.to_string(),
        Err(error) => error.to_string(),
    }
}
