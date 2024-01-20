use std::fmt::Debug;
use crate::errors::{ParseErr, ParseErrKind};
use crate::interpreter::Interpreter;
use crate::operations::Operation;
use crate::parser::Parser;
use crate::expressions::{Evaluable, Expr};
use crate::types::Type;

#[derive(Debug, Clone)]
pub enum Data {
    String(String),
    Number(f64),
    Bool(bool),
    TempNil // just a temporary null value in the meantime as we dont have empty tuples yet
}

impl Data {
    pub fn to_type(&self) -> Type {
        match self {
            Data::String(val) => Type::String,
            Data::Number(val) => Type::Number,
            Data::Bool(val) => Type::Bool,
            Data::TempNil => Type::TempNil
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
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        Ok(self.data.to_type())
    }

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
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let lhs = self.lhs.get_type(parser)?;
        let rhs = self.rhs.get_type(parser)?;

        self.operation.typ(&lhs, &rhs)
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let lhs = self.lhs.eval(interpreter);
        let rhs = self.rhs.eval(interpreter);

        self.operation.op(&lhs, &rhs)
    }
}

