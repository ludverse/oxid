use crate::data::Data;
use crate::types::Type;

pub mod print;

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

    pub fn arg_types(&self) -> &'static [(&'static str, Type)] {
        match self {
            BuiltinFunc::Print => print::arg_types()
        }
    }

    pub fn return_type(&self) -> Type {
        match self {
            BuiltinFunc::Print => print::return_type()
        }
    }

    pub fn eval(&self, args: Vec<Data>) -> Data {
        match self {
            BuiltinFunc::Print => print::eval(args)
        }
    }
}
