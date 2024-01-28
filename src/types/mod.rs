use crate::statements::r#fn::FunctionSignature;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Number,
    Bool,
    Fn(FunctionSignature),
    TempNil
}

impl Type {
    pub fn from_name(name: &String) -> Option<Type> {
        match &name[..] {
            "String" => Some(Type::String),
            "Number" => Some(Type::Number),
            "Bool" => Some(Type::Bool),
            "Fn" => Some(Type::Bool),
            _ => None
        }
    }
}
