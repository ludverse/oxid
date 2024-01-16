use crate::data::{Operation, Data};
use crate::errors::ParseErrKind;

impl Data {
    pub fn rem(&self, rhs: &Data) -> Result<Data, ParseErrKind> {
        match self {
            Data::Number(lhs) => match rhs {
                Data::Number(val) => Ok(Data::Number(lhs % val)),
                _ => Err(self.illegal_operation(Operation::Rem, rhs))
            },
            _ => Err(self.illegal_operation(Operation::Rem, rhs))
        }
    }
}
