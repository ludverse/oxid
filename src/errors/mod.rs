use crate::operations::Operation;
use crate::tokenizer::token::{Token, TokenPos};

#[derive(Debug, Clone)]
pub struct ParseErr {
    pub err_kind: ParseErrKind,
    pub token_pos: TokenPos
}

impl ParseErr {
    pub fn report(&self) -> ! {
        panic!(
            "{}: {} ({})",
            self.token_pos.filename,
            self.err_kind.err_msg(),
            self.token_pos
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrKind {
    UnexpectedChar(char),
    UnexpectedToken(String, String),
    UnknownField(),
    IncompatiableOperation(Operation, String, String),
    UnmatchedDelimiter(char),
    InvalidPathUse(String),
    NotCallable()
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
            ParseErrKind::UnknownField() => format!("unknown field `i cant be fucked lol`"),
            ParseErrKind::IncompatiableOperation(operation, lhs, rhs) => format!("incompatiable {:?} operation on {} and {}", operation, lhs, rhs),
            ParseErrKind::UnmatchedDelimiter(c) => format!("unmatched delimiter `{}`", c),
            ParseErrKind::InvalidPathUse(expr_type) => format!("cannot use {} as a path", expr_type),
            ParseErrKind::NotCallable() => format!("some expr is not callable icbf")
        }
    }
}

pub fn map_err_token<T>(res: Result<T, ParseErrKind>, token: &Token) -> Result<T, ParseErr> {
    res.map_err(|err_kind| err_kind.from_token(token))
}
