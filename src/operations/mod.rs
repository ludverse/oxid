use crate::errors::ParseErrKind;
use crate::data::Data;
use crate::types::Type;

pub mod mul;
pub mod div;
pub mod rem;
pub mod add;
pub mod sub;
pub mod eq;
pub mod gt;
pub mod gte;
pub mod lt;
pub mod lte;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Eq,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl Operation {
    pub fn typ(&self, lhs: &Type, rhs: &Type) -> Option<Type> {
        match self {
            Operation::Add => add::typ(lhs, rhs),
            Operation::Sub => sub::typ(lhs, rhs),
            Operation::Mul => mul::typ(lhs, rhs),
            Operation::Div => div::typ(lhs, rhs),
            Operation::Rem => rem::typ(lhs, rhs),
            Operation::Eq => eq::typ(lhs, rhs),
            Operation::Gt => gt::typ(lhs, rhs),
            Operation::Gte => gte::typ(lhs, rhs),
            Operation::Lt => lt::typ(lhs, rhs),
            Operation::Lte => lte::typ(lhs, rhs)
        }
    }

    pub fn op(&self, lhs: &Data, rhs: &Data) -> Data {
        let res = match self {
            Operation::Mul => mul::op(lhs, rhs),
            Operation::Div => div::op(lhs, rhs),
            Operation::Rem => rem::op(lhs, rhs),
            Operation::Add => add::op(lhs, rhs),
            Operation::Sub => sub::op(lhs, rhs),
            Operation::Eq => eq::op(lhs, rhs),
            Operation::Gt => gt::op(lhs, rhs),
            Operation::Gte => gte::op(lhs, rhs),
            Operation::Lt => lt::op(lhs, rhs),
            Operation::Lte => lte::op(lhs, rhs),
        };

        res.expect("invalid operation slipped through to the interpreter after a valid operation check")
    }
}
