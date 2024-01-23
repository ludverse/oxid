use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::ParseErrKind;
use crate::expressions::{Expr, Evaluable};
use crate::data::Data;
use crate::operations::Operation;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct ExprPath {
    path: Vec<String>
}

impl ExprPath {
    pub fn new(path: Vec<String>) -> ExprPath {
        ExprPath {
            path
        }
    }
}

impl Evaluable for ExprPath {
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        parser.sim_memory.get(&self.path[0])
            .ok_or(ParseErrKind::UnknownField(self.path[0].clone()))
            .cloned()
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        interpreter.memory.get(&self.path[0]).unwrap().clone()
    }
}

