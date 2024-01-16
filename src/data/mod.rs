use std::fmt::Debug;
use crate::errors::{ParseErr, ParseErrKind};
use crate::interpreter::Interpreter;
use crate::tokenizer::Token;
use crate::expressions::{Evaluable, Expr};

mod add;
mod rem;
mod eq;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq
}

#[derive(Debug, Clone)]
pub enum Data {
    String(String),
    Number(f64),
    Bool(bool)
}

impl Data {
    pub fn to_token_type(&self) -> Token {
        match self {
            Data::String(val) => Token::String(val.to_string()),
            Data::Number(val) => Token::Number(*val),
            Data::Bool(val) => Token::Bool(*val)
        }
    }

    pub fn op(&self, op: Operation, rhs: &Data) -> Result<Data, ParseErrKind> {
        match op {
            Operation::Add => self.add(rhs),
            Operation::Rem => self.rem(rhs),
            Operation::Eq => self.eq(rhs),
            _ => unimplemented!()
        }
    }

    fn illegal_operation(&self, op: Operation, rhs: &Data) -> ParseErrKind {
        ParseErrKind::InvalidOperation(op, format!("{:?}", self), format!("{:?}", rhs))
    }
}

#[derive(Debug, Clone)]
pub struct ExprLiteral {
    data: Data
}

impl ExprLiteral {
    pub fn new(data: Data) -> ExprLiteral {
        ExprLiteral {
            data
        }
    }
}

impl Evaluable for ExprLiteral {
    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        self.data.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ExprBinary {
    operation: Operation,
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl ExprBinary {
    pub fn new(operation: Operation, lhs: Box<Expr>, rhs: Box<Expr>) -> ExprBinary {
        ExprBinary {
            operation,
            lhs,
            rhs
        }
    }
}

impl Evaluable for ExprBinary {
    fn eval(&self, interpreter: &mut crate::interpreter::Interpreter) -> Data {
        let lhs = self.lhs.eval(interpreter);
        lhs.op(self.operation, &self.rhs.eval(interpreter)).unwrap()
    }
}

