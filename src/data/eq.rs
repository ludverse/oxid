use crate::data::{Operation, Data};
use crate::errors::ParseErrKind;

impl Data {
    pub fn eq(&self, rhs: &Data) -> Result<Data, ParseErrKind> {
        match self {
            Data::String(lhs) => match rhs {
                Data::String(val) => Ok(Data::Bool(lhs == val)),
                _ => Err(self.illegal_operation(Operation::Eq, rhs))
            },
            Data::Number(lhs) => match rhs {
                Data::Number(val) => Ok(Data::Bool(lhs == val)),
                _ => Err(self.illegal_operation(Operation::Eq, rhs))
            },
            _ => Err(self.illegal_operation(Operation::Eq, rhs))
        }
    }
}
