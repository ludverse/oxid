use std::fmt::Debug;
use crate::errors::{ParseErr, ParseErrKind};
use crate::interpreter::Interpreter;
use crate::operations::Operation;
use crate::parser::Parser;
use crate::expressions::{Evaluable, Expr};
use crate::statements::r#fn::FunctionDeclaration;
use crate::types::Type;

#[derive(Debug, Clone)]
pub enum Data {
    String(String),
    Number(f64),
    Bool(bool),
    Fn(FunctionDeclaration),
    TempNil // just a temporary null value in the meantime as we dont have empty tuples yet
}

impl Data {
    pub fn get_type(&self) -> Type {
        match &self {
            Data::String(val) => Type::String,
            Data::Number(val) => Type::Number,
            Data::Bool(val) => Type::Bool,
            Data::Fn(val) => Type::Fn { args: val.args.clone(), return_type: val.return_type.clone() },
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
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        Ok(self.data.get_type())
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
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let lhs = self.lhs.typ(parser)?;
        let rhs = self.rhs.typ(parser)?;

        self.operation.typ(&lhs, &rhs)
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let lhs = self.lhs.eval(interpreter);
        let rhs = self.rhs.eval(interpreter);

        self.operation.op(&lhs, &rhs)
    }
}

