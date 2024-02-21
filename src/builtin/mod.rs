use crate::data::Data;
use crate::errors::ParseErrKind;
use crate::types::Type;

pub mod print;

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinFunc {
    Print,
}

impl BuiltinFunc {
    pub fn from_name(name: &String) -> Option<BuiltinFunc> {
        match &name[..] {
            "print" => Some(BuiltinFunc::Print),
            _ => None
        }
    }

    pub fn type_check(&self, args: Vec<Type>) -> Result<Type, ParseErrKind>{
        match self {
            BuiltinFunc::Print => print::type_check(args)
        }
    }

    pub fn eval(&self, args: Vec<Data>) -> Data {
        match self {
            BuiltinFunc::Print => print::eval(args)
        }
    }
}
