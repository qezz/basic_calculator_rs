use types::Expr;
use types::Expr::*;

pub fn evaluate(expr: Expr) -> i32 {
    match expr {
        ENum(num) => num,
        EAdd(expr1, expr2) => evaluate(*expr1) + evaluate(*expr2),
    }
}
