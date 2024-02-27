use crate::builtin::BuiltinFn;
use crate::memory::Memory;
use crate::tokenizer::{token::Token, token_type::TokenType};
use crate::statements::Statement;
use crate::errors::{ParseErrKind, ParseErr};
use crate::types::Type;
pub use token_collectior::TokenCollector;

mod token_collectior;

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Statement>
}

impl Program {
    pub fn new(body: Vec<Statement>) -> Program {
        Program {
            body
        }
    }
}

pub struct Parser<'a> {
    pub collector: TokenCollector<'a>,
    pub sim_memory: Memory<Type>
}

impl<'a> Parser<'a> {
    pub fn new(collector: TokenCollector<'a>) -> Parser<'a> {
        let mut sim_memory = Memory::new();

        BuiltinFn::populate_sim_memory(&mut sim_memory);

        Parser {
            collector,
            sim_memory,
        }
    }

    pub fn unexpected_token(&self, token: &Token, expected: &str) -> ParseErr {
        ParseErrKind::UnexpectedToken(format!("{:?}", token.token), expected.to_string())
            .from_token(token)
    }

    pub fn generate_program(&mut self) -> Program {
        let mut statements = vec![];

        for _ in 0..1_000_000 {
            let next_token = self.collector.next();
            match next_token.token {
                TokenType::EOF => return Program::new(statements),
                _ => {
                    let statement = Statement::parse_statement(self, next_token).unwrap_or_else(|err| err.report());
                    statements.push(statement);
                }
            }
        }

        panic!("loop never breaked");
    }
}

