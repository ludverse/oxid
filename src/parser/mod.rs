use std::collections::HashMap;

use crate::expressions::{parse_expr_data, Expr};
use crate::expressions::data::ExprBinary;
use crate::tokenizer::Token;
use crate::statements::{Statement, Program, parse_statement};
use crate::errors::{ParseErrKind, ParseErr};
pub use token_collectior::TokenCollector;

mod token_collectior;

macro_rules! match_tree {
    ($a:expr, $b:pat, $c:expr, $d:expr, $e:expr) => {
        match $a {
            $b => $e,
            _ => Err($c.unexpected_token($d))
        }
    }
}
pub(crate) use match_tree;

pub struct Parser<'a> {
    pub collector: TokenCollector<'a>,
    pub sim_memory: HashMap<String, Box<Expr>>
}

impl<'a> Parser<'a> {
    pub fn new(collector: TokenCollector<'a>) -> Parser<'a> {
        Parser {
            collector,
            sim_memory: HashMap::new()
        }
    }

    pub fn unexpected_token(&self, expected: &str) -> ParseErr {
        let token = self.collector.current();
        let pos = self.collector.current_pos();

        ParseErrKind::UnexpectedToken(format!("{:?}", token), expected.to_string()).to_err(pos)
    }

    pub fn parse_expr(&mut self, first_token: &Token) -> Result<Box<Expr>, ParseErr> {
        let mut left = parse_expr_data(self, first_token)?;

        for i in 0..=1_000_000 {
            let operation = self.collector.next();
            match operation.to_operation() {
                Some(operation) => {
                    let expr_token = self.collector.next();
                    let right = parse_expr_data(self, expr_token)?;

                    left = Box::new(Expr::Binary(ExprBinary::new(operation, left, right)));
                },
                None => {
                    self.collector.back();
                    return Ok(left)
                }
            }
        }

        panic!("loop never breaked");
    }

    pub fn parse_block(&mut self) -> Result<Vec<Statement>, ParseErr> {
        let mut statements = vec![];

        for i in 0..1_000_000 {
            let next_token = self.collector.next();
            match next_token {
                Token::RightCurly => return Ok(statements),
                _ => {
                    statements.push(parse_statement(self, next_token)?);
                }
            }
        }

        panic!("loop never breaked");
    }

    pub fn generate_program(&mut self) -> Program {
        let mut statements = vec![];

        for i in 0..1_000_000 {
            let next_token = self.collector.next();
            dbg!(next_token);
            match next_token {
                Token::EOF => return Program::new(statements),
                _ => {
                    let statement = parse_statement(self, next_token).unwrap_or_else(|err| err.report());
                    statements.push(statement);
                }
            }
        }

        panic!("loop never breaked");
    }
}

