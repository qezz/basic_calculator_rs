use types::Expr;
use types::Expr::*;

pub fn evaluate(expr: Expr) -> f32 {
    match expr {
        ENum(num) => num,
        EAdd(expr1, expr2) => evaluate(*expr1) + evaluate(*expr2),
        ESub(expr1, expr2) => evaluate(*expr1) - evaluate(*expr2),
        EMul(expr1, expr2) => evaluate(*expr1) * evaluate(*expr2),
        EDiv(expr1, expr2) => evaluate(*expr1) / evaluate(*expr2),
    }
}
