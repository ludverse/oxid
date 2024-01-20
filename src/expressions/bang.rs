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
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        unimplemented!()
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        unimplemented!();
    }
}
