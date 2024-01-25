use crate::types::Type;

use super::Operation;

#[test]
fn eq_strings_type_check() {
    let res_type = Operation::Eq.typ(&Type::String, &Type::String);
    assert_eq!(res_type, Ok(Type::Bool));
}

#[test]
fn cannot_eq_different_types() {
    assert!(Operation::Eq.typ(&Type::String, &Type::Number).is_err())
}

#[test]
fn add_string_types() {
    let res_type = Operation::Add.typ(&Type::String, &Type::String);
    assert_eq!(res_type, Ok(Type::String));
}

#[test]
fn add_numbers_types() {
    let res_type = Operation::Add.typ(&Type::Number, &Type::Number);
    assert_eq!(res_type, Ok(Type::Number));
}

#[test]
fn cannot_add_different_types() {
    assert!(Operation::Add.typ(&Type::String, &Type::Number).is_err())
}

#[test]
fn sub_numbers_types() {
    let res_type = Operation::Sub.typ(&Type::Number, &Type::Number);
    assert_eq!(res_type, Ok(Type::Number));
}

#[test]
fn cannot_sub_different_types() {
    assert!(Operation::Sub.typ(&Type::String, &Type::Number).is_err())
}

#[test]
fn mul_numbers_types() {
    let res_type = Operation::Mul.typ(&Type::Number, &Type::Number);
    assert_eq!(res_type, Ok(Type::Number));
}

#[test]
fn cannot_mul_different_types() {
    assert!(Operation::Mul.typ(&Type::String, &Type::Number).is_err())
}

#[test]
fn div_numbers_types() {
    let res_type = Operation::Div.typ(&Type::Number, &Type::Number);
    assert_eq!(res_type, Ok(Type::Number));
}

#[test]
fn cannot_div_different_types() {
    assert!(Operation::Div.typ(&Type::String, &Type::Number).is_err())
}

#[test]
fn rem_numbers_types() {
    let res_type = Operation::Rem.typ(&Type::Number, &Type::Number);
    assert_eq!(res_type, Ok(Type::Number));
}

#[test]
fn cannot_rem_different_types() {
    assert!(Operation::Sub.typ(&Type::String, &Type::Number).is_err())
}
