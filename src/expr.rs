use std::collections::HashMap;
use std::fmt::Debug;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expressions::{ExprLiteral, ExprBinary};
use crate::expressions::identifier::{ExprAssign, ExprMethod, ExprPath};
use crate::expressions::r#for::ExprFor;
use crate::interpreter::Interpreter;
use crate::statements::Statement;
use crate::tokenizer::Token;

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
            true => Err(ParseErrKind::InvalidOperation(Operation::Add, self.clone(), rhs.clone())),
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
pub enum Expr {
    Literal(ExprLiteral),
    Binary(ExprBinary),
    Path(ExprPath),
    Method(ExprMethod),
    Assign(ExprAssign),
    For(ExprFor)
}

pub trait Evaluable {
    fn eval(&self, interpreter: &mut Interpreter) -> Data;
}

impl Expr {
    pub fn eval(&self, interpreter: &mut Interpreter) -> Data {
        match self {
            Expr::Literal(literal_expr) => literal_expr.eval(interpreter),
            Expr::Binary(binary_expr) => binary_expr.eval(interpreter),
            Expr::Path(path_expr) => path_expr.eval(interpreter),
            Expr::Method(expr_method) => unimplemented!(),
            Expr::Assign(assign_expr) => assign_expr.eval(interpreter),
            Expr::For(for_expr) => for_expr.eval(interpreter)
        }
    }
}

