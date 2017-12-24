use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Expr {
    ENum(f32),
    EVar(String),
    EAdd(Box<Expr>, Box<Expr>),
    ESub(Box<Expr>, Box<Expr>),
    EMul(Box<Expr>, Box<Expr>),
    EDiv(Box<Expr>, Box<Expr>),
    EExp(Box<Expr>, Box<Expr>),
    ELet(String, Box<Expr>),
    EDefun(String, String, Box<Expr>),
    EReturn(Box<Expr>),
}

pub struct Environment(pub HashMap<String, f32>);

impl Environment {
    pub fn new() -> Environment {
        Environment(HashMap::new())
    }
    pub fn get(&self, var_name: String) -> f32 {
        *self.0.get(&var_name).unwrap()
    }
    pub fn add(&mut self, var_name: String, result: f32) -> &mut Environment {
        &self.0.insert(var_name, result);
        self
    }
}
