use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::data::Data;
use crate::operations::Operation;
use crate::tokenizer::{TokenType, Token};
use crate::types::Type;

use super::path::ExprPath;

#[derive(Debug, Clone)]
pub struct ExprField {
    path: Box<Expr>
}

impl ExprField {
    pub fn new(parent: Box<Expr>, path: Box<Expr>) -> Self {
        Self {
            parent,
            path
        }
    }

    fn mangle(&self) -> String {
        let mut res = String::new();

        res.push_str(&self.parent.mangle()[..]);
        res.push('_');

        let path = self.path.replace("%", "%%")
            .replace("_", "%_");

        res.push_str(&path[..]);

        res
    }
}

impl Evaluable for ExprField {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        Ok(Type::TempNil)
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        Data::TempNil
    }
}
