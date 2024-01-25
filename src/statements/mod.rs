use std::fmt::Debug;

use crate::errors::ParseErr;
use crate::expressions::Expr;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::{Token, TokenType};

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
        let res = match first_token.token {
            TokenType::Let => VariableAssignment::parse(parser, first_token),
            TokenType::Fn => FunctionDeclaration::parse(parser, first_token),
            _ => Expr::parse(parser, first_token)
        };

        let semicolon_token = parser.collector.next();
        match semicolon_token.token {
            TokenType::Semicolon => res,
            _ => {
                let mut enforce_semicolon = true;

                if let Ok(res) = &res {
                    enforce_semicolon = !matches!(res,
                        Statement::FunctionDeclaration(_) |
                        Statement::Expr(Expr::For(_)) |
                        Statement::Expr(Expr::If(_))
                    );
                }

                if enforce_semicolon {
                    Err(parser.unexpected_token(semicolon_token, "Semicolon"))
                } else {
                    parser.collector.back();
                    res
                }
            }
        }
    }
}
