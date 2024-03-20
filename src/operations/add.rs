use crate::data::Data;
use crate::types::Type;

pub fn typ(lhs: &Type, rhs: &Type) -> Option<Type> {
    match lhs {
        Type::String => match rhs {
            Type::String => Some(Type::String),
            _ => None
        },
        Type::Number => match rhs {
            Type::Number => Some(Type::Number),
            _ => None
        },
        _ => None
    }
}

pub fn op(lhs: &Data, rhs: &Data) -> Option<Data> {
    match lhs {
        Data::String(lhs) => match rhs {
            Data::String(rhs) => Some(Data::String(lhs.clone() + rhs)),
            _ => None
        },
        Data::Number(lhs) => match rhs {
            Data::Number(rhs) => Some(Data::Number(*lhs + *rhs)),
            _ => None
        },
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Type;

    use super::super::Operation;

    #[test]
    fn add_numbers() {
        let res_type = Operation::Add.typ(&Type::Number, &Type::Number);
        assert_eq!(res_type, Some(Type::Number));
    }

    #[test]
    fn add_strings() {
        let res_type = Operation::Add.typ(&Type::String, &Type::String);
        assert_eq!(res_type, Some(Type::String));
    }

    #[test]
    fn cannot_add_different() {
        assert!(Operation::Add.typ(&Type::String, &Type::Number).is_none())
    }
}
