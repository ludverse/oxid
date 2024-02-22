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
            Self::Fn { args_types: args, return_type } => Some(String::from("Fn")),
            _ => None
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
