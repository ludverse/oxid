use crate::errors::ParseErrKind;
use crate::expressions::{Evaluable, Expr};
use crate::data::Data;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::types::Type;

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
    fn typ(&self, _parser: &Parser) -> Type {
        unimplemented!()
    }

    fn eval(&self, _interpreter: &mut Interpreter) -> Data {
        unimplemented!();
    }
}
