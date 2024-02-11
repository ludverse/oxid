use crate::statements::r#fn::FunctionSignature;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Number,
    Bool,
    Fn {
        args: Vec<(String, Type)>,
        return_value: Box<Type>
    },
    TempNil
}

impl Type {
    pub fn get_name(&self) -> Option<String> {
        match self {
            Self::String => Some(String::from("String")),
            Self::Number => Some(String::from("Number")),
            Self::Bool => Some(String::from("Bool")),
            Self::Fn { args, return_value } => Some(String::from("Fn")),
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
