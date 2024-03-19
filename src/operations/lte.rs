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
            Data::Number(rhs) => Some(Data::Bool(lhs <= rhs)),
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
    fn lte_numbers() {
        let res_type = Operation::Lte.typ(&Type::Number, &Type::Number);
        assert_eq!(res_type, Some(Type::Bool));
    }

    #[test]
    fn cannot_lte_different() {
        assert!(Operation::Lte.typ(&Type::String, &Type::Number).is_none())
    }
}
