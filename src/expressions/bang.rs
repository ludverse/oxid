use crate::{expressions::{Evaluable, Expr}, data::Data};

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Bang
}

#[derive(Debug, Clone)]
pub struct ExprUnary {
    op: UnaryOp,
    value: Box<Expr>
}

impl Evaluable for ExprUnary {
    fn eval(&self, interpreter: &mut crate::interpreter::Interpreter) -> Data {
        unimplemented!();
    }
}
