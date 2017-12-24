use std::collections::HashMap;

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
    EFunCall(String, Vec<Expr>),
    EDefun(String, Vec<String>, Vec<Expr>),
    EReturn(Box<Expr>),
}

#[derive(Clone)]
pub struct Environment(pub HashMap<String, Expr>);

impl Environment {
    pub fn new() -> Environment {
        Environment(HashMap::new())
    }
    pub fn get(&self, var_name: String) -> Expr {
        self.0.get(&var_name).unwrap().clone()
    }
    pub fn add(&mut self, var_name: String, result: Expr) -> &mut Environment {
        &self.0.insert(var_name, result);
        self
    }
}
