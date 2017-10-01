#[derive(Debug, PartialEq)]
pub enum Expr {
    ENum(i32),
    EAdd(Box<Expr>, Box<Expr>),
}
