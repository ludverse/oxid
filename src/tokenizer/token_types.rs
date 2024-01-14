use crate::report_pos;
use crate::expr::{
    Operation,
    Data
};

pub type TokenVec = Vec<(usize, Token)>;

#[derive(Debug, Clone)]
pub enum Token {
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

    Semicolon,
    Dot, Range,
    Comma,

    Fn, Let, Mut, For, In, If, Else, Match,

    Identifier(String),
    String(String),
    Number(f64),
    Bool(bool),

    EOF
}

impl Token {
    pub fn get_next_type(start_byte_i: usize, buf: &str) -> Option<(Token, usize)> {
        let buf = &buf[start_byte_i..];
        let mut chars = buf.chars();

        let mut token_len = 1;
        let mut do_decrease_len = false;
        let first_char = chars.next()?;

        let mut next_char = || {
            token_len += 1;
            chars.next()
        };
        let mut decrease_len = |default_type: Token| {
            do_decrease_len = true;
            default_type
        };

        let token_type = match first_char {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBrace,
            ']' => Token::RightBrace,
            '{' => Token::LeftCurly,
            '}' => Token::RightCurly,
            '|' => Token::Pipe,

            '+' => match next_char() {
                Some('=') => Token::PlusEqual,
                _ => decrease_len(Token::Plus)
            },
            '-' => match next_char() {
                Some('=') => Token::MinusEqual,
                _ => decrease_len(Token::Minus)
            },
            '*' => match next_char() {
                Some('=') => Token::StarEqual,
                _ => decrease_len(Token::Star)
            },
            '/' => match next_char() {
                Some('=') => Token::SlashEqual,
                _ => decrease_len(Token::Slash)
            },
            '%' => Token::Remainder,

            '=' => match next_char() {
                Some('=') => Token::EqualEqual,
                _ => decrease_len(Token::Equal)
            },
            '!' => match next_char() {
                Some('=') => Token::BangEqual,
                _ => decrease_len(Token::Bang)
            },
            '<' => match next_char() {
                Some('=') => Token::GreaterEqual,
                _ => decrease_len(Token::Greater)
            },
            '>' => match next_char() {
                Some('=') => Token::LessEqual,
                _ => decrease_len(Token::Less)
            },

            ';' => Token::Semicolon,
            '.' => match next_char() {
                Some('.') => Token::Range,
                _ => decrease_len(Token::Dot),
            },
            ',' => Token::Comma,

            '"' => {
                let mut current_char = next_char();
                while current_char.is_some() && current_char != Some('"') {
                    current_char = next_char();
                }

                if current_char.is_none() {
                    report_pos(302, "unmatched delimiter `\"`".to_string(), start_byte_i);
                }

                let string = &buf[1..token_len - 1].to_string();
                Token::String(string.to_string())
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
                    Token::Number(*number)
                } else if first_char.is_ascii_alphabetic() || first_char == '_' || first_char == '$' {
                    let mut current_char = next_char();
                    while current_char.is_some() && current_char.unwrap().is_ascii_alphabetic() || current_char == Some('_') || current_char == Some('$') {
                        current_char = next_char();
                    }

                    token_len -= 1;

                    let name = &buf[..token_len];
                    match name {
                        "fn" => Token::Fn,
                        "let" => Token::Let,
                        "mut" => Token::Mut,
                        "for" => Token::For,
                        "in" => Token::In,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "match" => Token::Match,
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        _ => Token::Identifier(name.to_string())
                    }
                } else {
                    report_pos(301, format!("unexpected character: `{}`", first_char), start_byte_i);
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
            _ => None
        }
    }
}

