use crate::data::Data;
use crate::types::Type;

pub fn typ(lhs: &Type, rhs: &Type) -> Option<Type> {
    match lhs {
        Type::Number => match rhs {
            Type::Number => Some(Type::Bool),
            _ => None
        },
        _ => None
    }
}

pub fn op(lhs: &Data, rhs: &Data) -> Option<Data> {
    match lhs {
        Data::Number(lhs) => match rhs {
            Data::Number(rhs) => Some(Data::Bool(lhs >= rhs)),
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
    fn gte_numbers() {
        let res_type = Operation::Gte.typ(&Type::Number, &Type::Number);
        assert_eq!(res_type, Ok(Type::Bool));
    }

    #[test]
    fn cannot_gte_different() {
        assert!(Operation::Gte.typ(&Type::String, &Type::Number).is_err())
    }
}
