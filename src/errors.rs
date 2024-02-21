use crate::{operations::Operation, tokenizer::Token};

pub struct ParseErr {
    err_kind: ParseErrKind,
    pos: usize
}

impl ParseErr {
    pub fn report(&self) -> ! {
        panic!("[{:?}] error: {} (char: {})", self.err_kind, self.err_kind.err_msg(), self.pos)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrKind {
    UnexpectedChar(char),
    UnexpectedToken(String, String),
    UnknownField(String),
    InvalidOperation(Operation, String, String),
    UnmatchedDelimiter(char),
    InvalidPathUse(String)
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
            ParseErrKind::UnexpectedChar(c) => format!("unexpected character `{}`", c),
            ParseErrKind::UnexpectedToken(got, expected) => format!("expected {}, got {}", expected, got),
            ParseErrKind::UnknownField(field) => format!("unknown field `{}`", field),
            ParseErrKind::InvalidOperation(operation, lhs, rhs) => format!("invalid {:?} on `{}` and `{}`", operation, lhs, rhs),
            ParseErrKind::UnmatchedDelimiter(c) => format!("unmatched delimiter `{}`", c),
            ParseErrKind::InvalidPathUse(expr_type) => format!("cannot use {} as a path", expr_type),
        }
    }
}

pub fn map_err_token<T>(res: Result<T, ParseErrKind>, token: &Token) -> Result<T, ParseErr> {
    res.map_err(|err_kind| err_kind.to_err(token.pos))
}
