use std::fmt;

use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct TokenPos {
    pub line_i: usize,
    pub col_i: usize,
    pub filename: String,
}

impl TokenPos {
    pub fn new(line_i: usize, col_i: usize, filename: String) -> Self {
        Self {
            line_i,
            col_i,
            filename,
        }
    }
}

impl fmt::Display for TokenPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line_i + 1, self.col_i + 1)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_pos: TokenPos,
    pub token: TokenType,
}

impl Token {
    pub fn new(token_pos: TokenPos, token: TokenType) -> Self {
        Self { token_pos, token }
    }

    pub fn next_token(token_pos: TokenPos, buf: &str) -> Option<(usize, Self)> {
        let next_token_data = TokenType::next_token_type(token_pos.clone(), buf)?;

        Some((next_token_data.1, Self::new(token_pos, next_token_data.0)))
    }
}
