use crate::expr::{Data, Operation};

pub struct ParseErr {
    err_kind: ParseErrKind,
    pos: usize
}

impl ParseErr {
    pub fn report(&self) -> ! {
        panic!("[{:?}] error: {} (char: {})", self.err_kind, self.err_kind.err_msg(), self.pos)
    }
}

#[derive(Debug)]
pub enum ParseErrKind {
    UnexpectedToken(String, String),
    UnknownField(String),
    InvalidOperation(Operation, Data, Data),
}

impl ParseErrKind {
    pub fn to_err(self, pos: usize) -> ParseErr {
        let ok = ParseErr {
            err_kind: self,
            pos
        };
        ok.report()
    }

    fn err_msg(&self) -> String {
        match self {
            ParseErrKind::UnexpectedToken(got, expected) => format!("expected {}, got {}", expected, got),
            ParseErrKind::UnknownField(field) => format!("unknown field `{}`", field),
            ParseErrKind::InvalidOperation(operation, lhs, rhs) => format!("invalid {:?} on `{:?}` and `{:?}`", operation, lhs, rhs)
        }
    }
}
