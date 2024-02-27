use crate::errors::ParseErrKind;
use crate::data::Data;
use crate::operations::Operation;
use crate::expressions::assign::AssignOp;

#[derive(Debug, Clone)]
pub struct TokenPos {

}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_pos: TokenPos,
    pub token: TokenType
}

impl Token {
    pub fn new(pos: usize, token: TokenType) -> Self {
        Self {
            token_pos: pos,
            token
        }
    }
}


#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftCurly, RightCurly,
    Pipe,

    Plus, PlusEqual,
    Minus, MinusEqual,
    Star, StarEqual,
    Slash, SlashEqual,
    Remainder,

    Equal, EqualEqual,
    Bang, BangEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Colon,
    Semicolon,
    Dot, Spread,
    Comma,

    Fn, Let, Mut, For, In, If, Else, Match,

    Identifier(String),
    String(String),
    Number(f64),
    Bool(bool),

    EOF
}

impl TokenType {
    pub fn get_next_type(start_byte_i: usize, buf: &str) -> Option<(TokenType, usize)> {
        let buf = &buf[start_byte_i..];
        let mut chars = buf.chars();

        let mut token_len = 1;
        let mut do_decrease_len = false;
        let first_char = chars.next()?;

        let mut next_char = || {
            token_len += 1;
            chars.next()
        };
        let mut decrease_len = |default_type: TokenType| {
            do_decrease_len = true;
            default_type
        };

        let token_type = match first_char {
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            '[' => Self::LeftBrace,
            ']' => Self::RightBrace,
            '{' => Self::LeftCurly,
            '}' => Self::RightCurly,
            '|' => Self::Pipe,

            '+' => match next_char() {
                Some('=') => Self::PlusEqual,
                _ => decrease_len(Self::Plus)
            },
            '-' => match next_char() {
                Some('=') => Self::MinusEqual,
                _ => decrease_len(Self::Minus)
            },
            '*' => match next_char() {
                Some('=') => Self::StarEqual,
                _ => decrease_len(Self::Star)
            },
            '/' => match next_char() {
                Some('=') => Self::SlashEqual,
                _ => decrease_len(Self::Slash)
            },
            '%' => Self::Remainder,

            '=' => match next_char() {
                Some('=') => Self::EqualEqual,
                _ => decrease_len(Self::Equal)
            },
            '!' => match next_char() {
                Some('=') => Self::BangEqual,
                _ => decrease_len(Self::Bang)
            },
            '<' => match next_char() {
                Some('=') => Self::GreaterEqual,
                _ => decrease_len(Self::Greater)
            },
            '>' => match next_char() {
                Some('=') => Self::LessEqual,
                _ => decrease_len(Self::Less)
            },

            ':' => Self::Colon,
            ';' => Self::Semicolon,
            '.' => match next_char() {
                Some('.') => Self::Spread,
                _ => decrease_len(Self::Dot),
            },
            ',' => Self::Comma,

            '"' => {
                let mut current_char = next_char();
                while current_char.is_some() && current_char != Some('"') {
                    current_char = next_char();
                }

                if current_char.is_none() {
                    // we gotta handle this sometime later
                    ParseErrKind::UnmatchedDelimiter('"').to_err(start_byte_i).report();
                }

                let string = &buf[1..token_len - 1].to_string();
                Self::String(string.to_string())
            },

            _ => {
                if first_char.is_digit(10) {
                    let mut current_char = next_char();
                    let mut has_decimal_point = false;
                    while current_char.is_some() && current_char.unwrap().is_digit(10) || current_char == Some('.') {
                        if current_char == Some('.') {
                            if has_decimal_point { break; };
                            has_decimal_point = true;
                        };
                        current_char = next_char();
                    }

                    token_len -= 1;

                    let number = &buf[..token_len];
                    if number.ends_with(".") { token_len -= 1 };

                    let number: &f64 = &buf[..token_len].parse().unwrap();
                    Self::Number(*number)
                } else if first_char.is_ascii_alphabetic() || first_char == '_' || first_char == '$' {
                    let mut current_char = next_char();
                    while current_char.is_some() && current_char.unwrap().is_ascii_alphabetic() || current_char == Some('_') || current_char == Some('$') {
                        current_char = next_char();
                    }

                    token_len -= 1;

                    let name = &buf[..token_len];
                    match name {
                        "fn" => Self::Fn,
                        "let" => Self::Let,
                        "mut" => Self::Mut,
                        "for" => Self::For,
                        "in" => Self::In,
                        "if" => Self::If,
                        "else" => Self::Else,
                        "match" => Self::Match,
                        "true" => Self::Bool(true),
                        "false" => Self::Bool(false),
                        _ => Self::Identifier(name.to_string())
                    }
                } else {
                    // we gotta handle this sometime later
                    ParseErrKind::UnexpectedChar(first_char).to_err(start_byte_i).report();
                }
            }
        };

        if do_decrease_len {
            token_len -= 1;
        }

        Some((token_type, token_len))
    }

    pub fn to_data(&self) -> Option<Data> {
        match self {
            Self::String(val) => Some(Data::String(val.to_string())),
            Self::Number(val) => Some(Data::Number(*val)),
            Self::Bool(val) => Some(Data::Bool(*val)),
            _ => None
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
            _ => None
        }
    }

    pub fn to_assign_op(&self) -> Option<AssignOp> {
        match self {
            Self::Equal => Some(AssignOp::Eq),
            Self::PlusEqual => Some(AssignOp::AddEq),
            _ => None
        }
    }
}

