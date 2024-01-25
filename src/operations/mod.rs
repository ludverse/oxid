use crate::errors::ParseErrKind;
use crate::data::Data;
use crate::types::Type;

pub mod eq;
pub mod add;
pub mod sub;
pub mod mul;
pub mod div;
pub mod rem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Rem
}

impl Operation {
    pub fn typ(&self, lhs: &Type, rhs: &Type) -> Result<Type, ParseErrKind> {
        let res = match self {
            Operation::Eq => eq::typ(lhs, rhs),
            Operation::Add => add::typ(lhs, rhs),
            Operation::Sub => sub::typ(lhs, rhs),
            Operation::Mul => mul::typ(lhs, rhs),
            Operation::Div => div::typ(lhs, rhs),
            Operation::Rem => rem::typ(lhs, rhs)
        };

        res.ok_or(ParseErrKind::InvalidOperation(*self, format!("{:?}", lhs), format!("{:?}", rhs)))
    }

    pub fn op(&self, lhs: &Data, rhs: &Data) -> Data {
        let res = match self {
            Operation::Eq => eq::op(lhs, rhs),
            Operation::Add => add::op(lhs, rhs),
            Operation::Sub => sub::op(lhs, rhs),
            Operation::Mul => mul::op(lhs, rhs),
            Operation::Div => div::op(lhs, rhs),
            Operation::Rem => rem::op(lhs, rhs)
        };

        res.expect("invalid operation slipped through to the interpreter after a valid operation check")
    }
}

#[cfg(test)]
mod tests;
