use crate::data::Data;
use crate::types::Type;

pub fn typ(lhs: &Type, rhs: &Type) -> Option<Type> {
    match lhs {
        Type::String => match rhs {
            Type::String => Some(Type::Bool),
            _ => None
        },
        Type::Number => match rhs {
            Type::Number => Some(Type::Bool),
            _ => None
        },
        _ => None
    }
}

pub fn op(lhs: &Data, rhs: &Data) -> Option<Data> {
    match lhs {
        Data::String(lhs) => match rhs {
            Data::String(rhs) => Some(Data::Bool(lhs == rhs)),
            _ => None
        },
        Data::Number(lhs) => match rhs {
            Data::Number(rhs) => Some(Data::Bool(lhs == rhs)),
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
    fn eq_numbers() {
        let res_type = Operation::Eq.typ(&Type::Number, &Type::Number);
        assert_eq!(res_type, Some(Type::Bool));
    }

    #[test]
    fn eq_strings() {
        let res_type = Operation::Eq.typ(&Type::String, &Type::String);
        assert_eq!(res_type, Some(Type::Bool));
    }

    #[test]
    fn cannot_eq_different() {
        assert!(Operation::Eq.typ(&Type::String, &Type::Number).is_none())
    }
}
