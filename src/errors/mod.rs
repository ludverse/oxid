use crate::operations::Operation;
use crate::tokenizer::token::{Token, TokenPos};

pub struct ParseErr {
    pub err_kind: ParseErrKind,
    pub token_pos: TokenPos
}

impl ParseErr {
    pub fn report(&self) -> ! {
        panic!(
            "error: {} ({})",
            self.err_kind.err_msg(),
            self.token_pos
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrKind {
    UnexpectedChar(char),
    UnexpectedToken(String, String),
    UnknownField(String),
    InvalidOperation(Operation, String, String),
    UnmatchedDelimiter(char),
    InvalidPathUse(String),
    NotCallable(String)
}

impl ParseErrKind {
    pub fn to_err(self, token_pos: TokenPos) -> ParseErr {
        let ok = ParseErr {
            err_kind: self,
            token_pos
        };
        ok.report();
    }

    pub fn from_token(self, token: &Token) -> ParseErr {
        let ok = ParseErr {
            err_kind: self,
            token_pos: token.token_pos.clone()
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
            ParseErrKind::NotCallable(mangled_path) => format!("`{}` is not callable", mangled_path)
        }
    }
}

pub fn map_err_token<T>(res: Result<T, ParseErrKind>, token: &Token) -> Result<T, ParseErr> {
    res.map_err(|err_kind| err_kind.from_token(token))
}
