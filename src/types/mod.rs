use crate::builtin::BuiltinFn;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Number,
    Bool,
    Fn {
        args_types: Vec<(String, Type)>,
        return_type: Box<Type>
    },
    BuiltinFn(BuiltinFn),
    TempNil
}

impl Type {
    pub fn get_name(&self) -> Option<String> {
        match self {
            Self::String => Some(String::from("String")),
            Self::Number => Some(String::from("Number")),
            Self::Bool => Some(String::from("Bool")),
            Self::Fn { args_types: _args, return_type: _ } => Some(String::from("Fn")),
            Self::BuiltinFn(_) => Some(String::from("Builtin")),
            Self::TempNil => Some(String::from("temporary nil val"))
        }
    }

    pub fn from_name(name: &String) -> Option<Type> {
        match &name[..] {
            "String" => Some(Self::String),
            "Number" => Some(Self::Number),
            "Bool" => Some(Self::Bool),
            "Fn" => unimplemented!(), // TODO
            _ => None
        }
    }
}
