use std::collections::HashMap;

use crate::statements::r#fn::FunctionSignature;
use crate::tokenizer::Token;
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
    pub sim_memory: HashMap<String, Type>,
    pub functions: HashMap<String, FunctionSignature>
}

impl<'a> Parser<'a> {
    pub fn new(collector: TokenCollector<'a>) -> Parser<'a> {
        Parser {
            collector,
            sim_memory: HashMap::new(),
            functions: HashMap::new()
        }
    }

    pub fn unexpected_token(&self, expected: &str) -> ParseErr {
        let token = self.collector.current();
        let pos = self.collector.current_pos();

        ParseErrKind::UnexpectedToken(format!("{:?}", token), expected.to_string()).to_err(pos)
    }

    pub fn generate_program(&mut self) -> Program {
        let mut statements = vec![];

        for _ in 0..1_000_000 {
            let next_token = self.collector.next();
            match next_token {
                Token::EOF => return Program::new(statements),
                _ => {
                    let statement = Statement::parse(self, next_token).unwrap_or_else(|err| err.report());
                    statements.push(statement);
                }
            }
        }

        panic!("loop never breaked");
    }
}

