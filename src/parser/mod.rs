use crate::memory::Memory;
use crate::tokenizer::{token::Token, token_type::TokenType};
use crate::statements::Statement;
use crate::errors::{ParseErrKind, ParseErr};
use crate::types::Type;

pub use token_collectior::TokenCollector;

mod token_collectior;

pub struct Parser<'a, 'm> {
    pub collector: TokenCollector<'a>,
    pub sim_memory: &'m mut Memory<Type>
}

impl<'a, 'm> Parser<'a, 'm> {
    pub fn new(collector: TokenCollector<'a>, sim_memory: &'m mut Memory<Type>) -> Self {
        Self {
            collector,
            sim_memory
        }
    }

    pub fn unexpected_token(&self, token: &Token, expected: &str) -> ParseErr {
        ParseErrKind::UnexpectedToken(format!("{:?}", token.token), expected.to_string())
            .from_token(token)
    }

    pub fn generate_program(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        for _ in 0..1_000_000 {
            let next_token = self.collector.next();
            match next_token.token {
                TokenType::EOF => return statements,
                _ => {
                    let statement = Statement::parse_statement(self, next_token).unwrap_or_else(|err| err.report());
                    statements.push(statement);
                }
            }
        }

        panic!("loop never breaked");
    }
}

