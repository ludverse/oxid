use crate::errors::{ParseErrKind};
use crate::expressions::{Evaluable, Expr};
use crate::data::Data;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct ExprIndex {
    index: Box<Expr>,
    value: Box<Expr>
}

impl ExprIndex {
    pub fn new(index: Box<Expr>, value: Box<Expr>) -> ExprIndex {
        ExprIndex {
            index,
            value
        }
    }
}

impl Evaluable for ExprIndex {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        self.value.typ(parser)
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        match self.index.eval(interpreter) {
            Data::Number(index) => {

                match self.value.eval(interpreter) {
                    Data::String(value) => Data::String(value.chars().collect::<Vec<_>>()[index as usize].to_string()), //very unsafe but idc
                    _ => unreachable!()
                }

            },
            _ => unreachable!()
        }
    }
}
