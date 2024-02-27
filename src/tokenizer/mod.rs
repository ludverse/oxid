use token::{Token, TokenPos};
use token_type::TokenType;

mod next_token;
pub mod token;
pub mod token_type;

pub fn tokenize(filename: &String, buf: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut line_i = 0;
    let mut col_i = 0;

    for line in buf.lines() {
        col_i = 0;

        let line_chars: Vec<_> = line.char_indices().collect();

        while col_i < line_chars.len() {
            let (byte_i, c) = line_chars[col_i];
            if c.is_whitespace() {
                col_i += 1;
                continue;
            };

            let token_data = Token::next_token(
                TokenPos::new(line_i, col_i, filename.to_string()),
                &line[byte_i..],
            );

            if let Some((char_len, token)) = token_data {
                col_i += char_len;
                tokens.push(token);
            } else {
                break;
            }
        }

        line_i += 1;
    }

    tokens.push(Token::new(
        TokenPos::new(line_i, 0, filename.to_string()),
        TokenType::EOF
    ));
    tokens
}

