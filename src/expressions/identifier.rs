use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expr::{Expr, Evaluable, Data};
use crate::tokenizer::Token;

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
    fn eval(&self, interpreter: &mut crate::interpreter::Interpreter) -> Data {
        interpreter.memory.get(&self.path[0]).unwrap().clone()
    }
}

#[derive(Debug, Clone)]
pub struct ExprMethod {
    path: Vec<String>,
    args: Vec<Box<Expr>>
}

impl ExprMethod {
    pub fn new(path: Vec<String>) -> ExprMethod {
        ExprMethod {
            path,
            args: vec![]
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprAssign {
    path: Vec<String>,
    value: Box<Expr>
}

impl ExprAssign {
    pub fn new(path: Vec<String>, value: Box<Expr>) -> ExprAssign {
        ExprAssign {
            path,
            value
        }
    }
}

impl Evaluable for ExprAssign {
    fn eval(&self, interpreter: &mut crate::interpreter::Interpreter) -> Data {
        let data = self.value.eval(interpreter);
        interpreter.memory.insert(self.path[0].to_string(), data.clone());

        data
    }
}

pub fn parse(parser: &mut Parser, name: &String) -> Result<Box<Expr>, ParseErr> {
    match parser.collector.next() {
        Token::Equal => {
            let expr_token = parser.collector.next();
            let expr = parser.parse_expr(expr_token)?;

            if !parser.sim_memory.contains_key(name) {
                return Err(ParseErrKind::UnknownField(name.to_string()).to_err(parser.collector.current_pos()));
            }
            return Ok(Box::new(Expr::Assign(ExprAssign::new(vec![name.to_string()], expr))));
        },
        _ => {
            parser.collector.back();

            if !parser.sim_memory.contains_key(name) {
                return Err(ParseErrKind::UnknownField(name.to_string()).to_err(parser.collector.current_pos()));
            }
            return Ok(Box::new(Expr::Path(ExprPath::new(vec![name.to_string()]))));
        }
    }
}

