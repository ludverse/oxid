use std::fmt::Debug;
use crate::builtin::BuiltinFn;
use crate::errors::ParseErrKind;
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
    BuiltinFn(BuiltinFn),
    TempNil // just a temporary null value in the meantime as we dont have empty tuples yet
}

impl Data {
    pub fn get_type(&self) -> Type {
        match self {
            Data::String(_val) => Type::String,
            Data::Number(_val) => Type::Number,
            Data::Bool(_val) => Type::Bool,
            Data::Fn(val) => Type::Fn { args_types: val.args.clone(), return_type: val.return_type.clone() },
            Data::BuiltinFn(builtin_fn) => Type::BuiltinFn(builtin_fn.clone()),
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
    fn type_check(&self, _parser: &Parser) -> Type {
        self.data.get_type()
    }

    fn eval(&self, _interpreter: &mut Interpreter) -> Data {
        self.data.clone()
    }
}

