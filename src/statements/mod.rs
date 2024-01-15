use std::fmt::Debug;

use crate::errors::ParseErr;
use crate::expressions::Expr;
use crate::parser::Parser;
use crate::tokenizer::Token;

pub mod r#let;

pub fn parse_statement(parser: &mut Parser, first_token: &Token) -> Result<Statement, ParseErr> {
    let mut enforce_semicolon = true;

    let res = match first_token {
        Token::Let => r#let::parse(parser),
        _ => {
            enforce_semicolon = false;

            let expr = parser.parse_expr(first_token)?;
            Ok(Statement::Expr(expr))
        }
    };

    match parser.collector.next() {
        Token::Semicolon => res,
        _ => {
            if enforce_semicolon {
                Err(parser.unexpected_token("Semicolon"))
            } else {
                parser.collector.back();
                res
            }
        }
    }
}

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

#[derive(Debug, Clone)]
pub enum Statement {
    VariableAssignment(r#let::VariableAssignment),
    Expr(Expr)
}

