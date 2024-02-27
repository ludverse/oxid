use crate::data::Data;
use crate::expressions::assign::AssignOp;
use crate::operations::Operation;

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftCurly,
    RightCurly,
    Pipe,

    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Remainder,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Colon,
    Semicolon,
    Dot,
    Spread,
    Comma,

    Fn,
    Let,
    Mut,
    For,
    In,
    If,
    Else,
    Match,

    Identifier(String),
    String(String),
    Number(f64),
    Bool(bool),

    EOF,
}

impl TokenType {
    pub fn to_data(&self) -> Option<Data> {
        match self {
            Self::String(val) => Some(Data::String(val.to_string())),
            Self::Number(val) => Some(Data::Number(*val)),
            Self::Bool(val) => Some(Data::Bool(*val)),
            _ => None,
        }
    }

    pub fn to_operation(&self) -> Option<Operation> {
        match self {
            Self::Plus => Some(Operation::Add),
            Self::Minus => Some(Operation::Sub),
            Self::Star => Some(Operation::Mul),
            Self::Slash => Some(Operation::Div),
            Self::Remainder => Some(Operation::Rem),
            Self::EqualEqual => Some(Operation::Eq),
            _ => None,
        }
    }

    pub fn to_assign_op(&self) -> Option<AssignOp> {
        match self {
            Self::Equal => Some(AssignOp::Eq),
            Self::PlusEqual => Some(AssignOp::AddEq),
            _ => None,
        }
    }
}
