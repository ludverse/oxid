use crate::data::{Operation, Data};
use crate::errors::ParseErrKind;

impl Data {
    pub fn add(&self, rhs: &Data) -> Result<Data, ParseErrKind> {
        match self {
            Data::String(lhs) => match rhs {
                Data::String(val) => Ok(Data::String(lhs.clone() + val)),
                _ => Err(self.illegal_operation(Operation::Add, rhs))
            },
            Data::Number(lhs) => match rhs {
                Data::Number(val) => Ok(Data::Number(*lhs + *val)),
                _ => Err(self.illegal_operation(Operation::Add, rhs))
            },
            _ => Err(self.illegal_operation(Operation::Add, rhs))
        }
    }
}
