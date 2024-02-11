use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::data::Data;
use crate::tokenizer::Token;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct ExprPath {
    parent: Option<Box<ExprPath>>,
    path: String
}

impl ExprPath {
    pub fn new(parent: Option<Box<ExprPath>>, path: String) -> Self {
        Self {
            parent,
            path
        }
    }

    fn mangle(&self) -> String {
        let mut res = String::new();

        if let Some(parent) = self.parent {
            res.push_str(&parent.mangle()[..]);
            res.push('_');
        }

        let path = self.path.replace("%", "%%")
            .replace("_", "%_");

        res.push_str(&path[..]);

        res
    }
}

impl Evaluable for ExprPath {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        parser.sim_memory.get(&self.mangle())
            .ok_or(ParseErrKind::UnknownField(self.path.clone()))
            .cloned()
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        interpreter.memory.get(&self.mangle()).unwrap().clone()
    }
}

pub fn parse(parser: &mut Parser, _first_token: &Token, expr: Expr) -> Result<Expr, ParseErr> {
    let next_token = parser.collector.next();
    match &next_token.token {
        TokenType::Identifier(path) => {

            match expr 
            Ok(Expr::Path(ExprPath::new(Some(Box::new(expr)), path.to_string())))
        },
        _ => Err(parser.unexpected_token(next_token, "field"))
    }
}
