use crate::helpers::destructive_loop;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::data::Data;
use crate::operations::Operation;
use crate::tokenizer::{TokenType, Token};
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct ExprField {
    pub field_name: String,
    pub child: Option<Box<Expr>>
}

impl ExprField {
    pub fn new(field_name: String, child: Option<Box<Expr>>) -> Self {
        Self {
            field_name,
            child
        }
    }
}

impl Evaluable for ExprField {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let mangled = self.mangle_path()?;
        parser.sim_memory.get(&mangled)
            .ok_or(ParseErrKind::UnknownField(mangled))
            .cloned()
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let mangled = self.mangle_path().unwrap();
        interpreter.memory.get(&mangled).unwrap().clone()
    }

    fn mangle_path(&self) -> Result<String, ParseErrKind> {
        if let Some(child) = &self.child {
            Ok(format!("{}.{}", child.mangle_path()?, self.field_name))
        } else {
            Ok(self.field_name.to_string())
        }
    }
}
