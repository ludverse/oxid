use crate::tokenizer::{
    Token,
    TokenVec
};

pub struct TokenCollector<'a> {
    pub tokens: &'a TokenVec,
    pub index: Option<usize>
}

impl<'a> TokenCollector<'a> {
    pub fn new(tokens: &'a TokenVec) -> TokenCollector<'a> {
        TokenCollector {
            tokens,
            index: None
        }
    }

    pub fn next(&mut self) -> &'a Token {
        if let Some(index) = self.index {
            self.index = Some(index + 1);
        } else {
            self.index = Some(0);
        }

        &self.tokens.get(self.index.unwrap()).expect("try to call next after EOF").1
    }

    pub fn back(&mut self) -> Option<&'a Token> {
        let index = self.index?;
        if index > 0 {
            self.index = Some(index - 1);
        } else {
            self.index = None;
        }

        Some(&self.tokens.get(self.index.unwrap()).expect("try to call back after EOF").1)
    }

    pub fn current(&self) -> &'a Token {
        &self.tokens.get(self.index.unwrap()).unwrap().1
    }

    pub fn current_pos(&self) -> usize {
        self.tokens.get(self.index.unwrap()).unwrap().0
    }
}

