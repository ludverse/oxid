use std::fmt::Debug;
use crate::errors::{ParseErr, ParseErrKind};
use crate::tokenizer::Token;
use crate::expressions::{Evaluable, Expr};

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
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
            Self::String(val) => Token::String(val.to_string()),
            Self::Number(val) => Token::Number(*val),
            Self::Bool(val) => Token::Bool(*val)
        }
    }

    pub fn add(&mut self, rhs: &Data) -> Result<(), ParseErrKind> {
        let mut do_return_compatability_err = false;

        match self {
            Self::String(ref mut lhs) => match rhs {
                Self::String(val) => lhs.push_str(&val[..]),
                _ => do_return_compatability_err = true
            },
            Self::Number(ref mut lhs) => match rhs {
                Self::Number(val) => *lhs += val,
                _ => do_return_compatability_err = true
            },
            _ => do_return_compatability_err = true 
        };

        match do_return_compatability_err {
            true => Err(ParseErrKind::InvalidOperation(Operation::Add, format!("{:?}", self), format!("{:?}", rhs))),
            false => Ok(())
        }
    }

    pub fn op(&mut self, op: Operation, rhs: &Data) -> Result<(), ParseErrKind> {
        match op {
            Operation::Add => self.add(rhs),
            _ => unimplemented!()
        }
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
    fn eval(&self, interpreter: &mut crate::interpreter::Interpreter) -> Data {
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
        let mut lhs = self.lhs.eval(interpreter);
        lhs.op(self.operation, &self.rhs.eval(interpreter)).unwrap();

        lhs
    }
}

