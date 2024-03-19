use crate::data::Data;
use crate::types::Type;

pub fn typ(lhs: &Type, rhs: &Type) -> Option<Type> {
    match lhs {
        Type::Number => match rhs {
            Type::Number => Some(Type::Number),
            _ => None
        },
        _ => None
    }
}

pub fn op(lhs: &Data, rhs: &Data) -> Option<Data> {
    match lhs {
        Data::Number(lhs) => match rhs {
            Data::Number(rhs) => Some(Data::Number(lhs % rhs)),
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
    fn rem_numbers() {
        let res_type = Operation::Rem.typ(&Type::Number, &Type::Number);
        assert_eq!(res_type, Some(Type::Number));
    }

    #[test]
    fn cannot_rem_different() {
        assert!(Operation::Rem.typ(&Type::String, &Type::Number).is_none())
    }
}
