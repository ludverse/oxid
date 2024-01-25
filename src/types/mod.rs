#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Number,
    Bool,
    TempNil
}

impl Type {
    pub fn from_name(name: &String) -> Option<Type> {
        match &name[..] {
            "String" => Some(Type::String),
            "Number" => Some(Type::Number),
            "Bool" => Some(Type::Bool),
            _ => None
        }
    }
}
