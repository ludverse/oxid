use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::data::Data;

use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;
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
    fn type_check(&self, parser: &Parser) -> Type {
        let mangled = self.mangle_path().unwrap();
        parser.sim_memory.get(&mangled).unwrap().clone()
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

pub fn parse(parser: &mut Parser, first_token: &Token, expr: Option<Expr>, field_name: &String) -> Result<Expr, ParseErr> {
    let child = expr.and_then(|expr| Some(Box::new(expr)));
    let expr_field = ExprField::new(field_name.to_string(), child);

    let mangled = expr_field.mangle_path().unwrap();

    if !parser.sim_memory.has(&mangled) {
        return Err(ParseErrKind::UnknownField().from_token(first_token));
    }

    Ok(Expr::Field(expr_field))
}
