use crate::errors::ParseErrKind;

use super::{token::TokenPos, token_type::TokenType};

impl TokenType {
    pub fn next_token_type(token_pos: TokenPos, buf: &str) -> Option<(TokenType, usize)> {
        let mut chars = buf.chars();

        let mut token_char_len = 1;
        let mut do_decrease_len = false;
        let first_char = chars.next()?;

        let mut next_char = || {
            token_char_len += 1;
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
                _ => decrease_len(Self::Plus),
            },
            '-' => match next_char() {
                Some('=') => Self::MinusEqual,
                _ => decrease_len(Self::Minus),
            },
            '*' => match next_char() {
                Some('=') => Self::StarEqual,
                _ => decrease_len(Self::Star),
            },
            '/' => match next_char() {
                Some('=') => Self::SlashEqual,
                _ => decrease_len(Self::Slash),
            },
            '%' => Self::Remainder,

            '=' => match next_char() {
                Some('=') => Self::EqualEqual,
                _ => decrease_len(Self::Equal),
            },
            '!' => match next_char() {
                Some('=') => Self::BangEqual,
                _ => decrease_len(Self::Bang),
            },
            '<' => match next_char() {
                Some('=') => Self::GreaterEqual,
                _ => decrease_len(Self::Greater),
            },
            '>' => match next_char() {
                Some('=') => Self::LessEqual,
                _ => decrease_len(Self::Less),
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
                    // TODO: we gotta handle this sometime later
                    ParseErrKind::UnmatchedDelimiter('"')
                        .to_err(token_pos)
                        .report();
                }

                let string = &buf[1..token_char_len - 1].to_string();
                Self::String(string.to_string())
            }

            _ => {
                if first_char.is_digit(10) {
                    let mut current_char = next_char();
                    let mut has_decimal_point = false;
                    while current_char.is_some() && current_char.unwrap().is_digit(10)
                        || current_char == Some('.')
                    {
                        if current_char == Some('.') {
                            if has_decimal_point {
                                break;
                            };
                            has_decimal_point = true;
                        };
                        current_char = next_char();
                    }

                    token_char_len -= 1;

                    let number = &buf[..token_char_len];
                    if number.ends_with(".") {
                        token_char_len -= 1
                    };

                    let number: &f64 = &buf[..token_char_len].parse().unwrap();
                    Self::Number(*number)
                } else if first_char.is_ascii_alphabetic() || first_char == '_' || first_char == '$'
                {
                    let mut current_char = next_char();
                    while current_char.is_some() && current_char.unwrap().is_ascii_alphabetic()
                        || current_char == Some('_')
                        || current_char == Some('$')
                    {
                        current_char = next_char();
                    }

                    token_char_len -= 1;

                    let name = &buf[..token_char_len];
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
                        _ => Self::Identifier(name.to_string()),
                    }
                } else {
                    // TODO: we gotta handle this sometime later
                    ParseErrKind::UnexpectedChar(first_char)
                        .to_err(token_pos)
                        .report();
                }
            }
        };

        if do_decrease_len {
            token_char_len -= 1;
        }

        Some((token_type, token_char_len))
    }
}
