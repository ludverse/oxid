pub use token_types::*;

mod token_types;

pub fn tokenize(buf: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let buf_char_indicies: Vec<_> = buf.char_indices().collect();

    let mut char_i = 0;
    while char_i < buf.len() {
        let (byte_i, c) = buf_char_indicies[char_i];
        if c.is_whitespace() {
            char_i += 1;
            continue;
        };

        let token = TokenType::get_next_type(byte_i, buf);

        if let Some(token) = token {
            char_i += token.1;

            tokens.push(Token::new(byte_i, token.0));
        } else {
            break;
        }
    }

    tokens.push(Token::new(buf.len(), TokenType::EOF));
    tokens
}

