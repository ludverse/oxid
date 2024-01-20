use std::fmt::Debug;

use crate::errors::ParseErr;
use crate::expressions::Expr;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::Token;
use r#let::VariableAssignment;
use r#fn::FunctionDeclaration;

pub mod r#let;
pub mod r#fn;

pub trait Executable {
    fn exec(&self, interpreter: &mut Interpreter);
}

pub trait ParseableStatement {
    fn parse(parser: &mut Parser, first_token: &Token) -> Result<Statement, ParseErr>;
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableAssignment(VariableAssignment),
    FunctionDeclaration(FunctionDeclaration),
    Expr(Expr)
}

impl Statement {
    pub fn exec(&self, interpreter: &mut Interpreter) {
        match self {
            Statement::VariableAssignment(var_assign) => var_assign.exec(interpreter),
            Statement::FunctionDeclaration(func_decl) => func_decl.exec(interpreter),
            Statement::Expr(expr) => expr.exec(interpreter)
        }
    }

    pub fn parse(parser: &mut Parser, first_token: &Token) -> Result<Statement, ParseErr> {
        let mut enforce_semicolon = true;

        let res = match first_token {
            Token::Let => VariableAssignment::parse(parser, first_token),
            Token::Fn => FunctionDeclaration::parse(parser, first_token),
            _ => {
                enforce_semicolon = false;

                Expr::parse(parser, first_token)
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
}
