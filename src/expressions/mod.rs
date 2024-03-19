use std::fmt::Debug;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind, map_err_token};
use crate::statements::{Executable, ParseableStatement, Statement};
use crate::tokenizer::{token::Token, token_type::TokenType};
use crate::data::{Data, ExprLiteral};
use crate::types::Type;
use crate::helpers::destructive_loop;

use binary::ExprBinary;
use index::ExprIndex;
use bang::ExprUnary;
use path::ExprField;
use call::ExprCall;
use assign::ExprAssign;
use block::ExprBlock;
use r#for::ExprFor;
use r#if::ExprIf;

pub mod binary;
pub mod index;
pub mod bang;
pub mod path;
pub mod call;
pub mod assign;
pub mod block;
pub mod r#for;
pub mod r#if;

pub trait Evaluable: Debug {
    fn typ(&self, parser: &Parser) -> Type;

    fn eval(&self, interpreter: &mut Interpreter) -> Data;

    fn mangle_path(&self) -> Result<String, ParseErrKind> {
        Err(ParseErrKind::InvalidPathUse(format!("{:?}", self)))
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(ExprLiteral),
    Binary(ExprBinary),
    Index(ExprIndex),
    Unary(ExprUnary),
    Field(ExprField),
    Call(ExprCall),
    Assign(ExprAssign),
    Block(ExprBlock),
    For(ExprFor),
    If(ExprIf)
}

impl Expr {
    pub fn typ(&self, parser: &Parser) -> Type {
        match self {
            Expr::Literal(literal_expr) => literal_expr.typ(parser),
            Expr::Binary(binary_expr) => binary_expr.typ(parser),
            Expr::Index(index_expr) => index_expr.typ(parser),
            Expr::Unary(_) => unimplemented!(),
            Expr::Field(field_expr) => field_expr.typ(parser),
            Expr::Call(call_expr) => call_expr.typ(parser),
            Expr::Assign(assign_expr) => assign_expr.typ(parser),
            Expr::Block(block_expr) => block_expr.typ(parser),
            Expr::For(for_expr) => for_expr.typ(parser),
            Expr::If(if_expr) => if_expr.typ(parser)
        }
    }

    pub fn eval(&self, interpreter: &mut Interpreter) -> Data {
        match self {
            Expr::Literal(literal_expr) => literal_expr.eval(interpreter),
            Expr::Binary(binary_expr) => binary_expr.eval(interpreter),
            Expr::Index(index_expr) => index_expr.eval(interpreter),
            Expr::Unary(_) => unimplemented!(),
            Expr::Field(field_expr) => field_expr.eval(interpreter),
            Expr::Call(call_expr) => call_expr.eval(interpreter),
            Expr::Assign(assign_expr) => assign_expr.eval(interpreter),
            Expr::Block(block_expr) => block_expr.eval(interpreter),
            Expr::For(for_expr) => for_expr.eval(interpreter),
            Expr::If(if_expr) => if_expr.eval(interpreter)
        }
    }

    pub fn mangle_path(&self) -> Result<String, ParseErrKind> {
        match self {
            Expr::Field(field_expr) => field_expr.mangle_path(),
            _ => Err(ParseErrKind::InvalidPathUse(format!("{:?}", self)))
        }
    }

    fn parse_expr_side(parser: &mut Parser, first_token: &Token) -> Result<Expr, ParseErr> {
        if let Some(data) = first_token.token.to_data() {
            let literal_expr = ExprLiteral::new(data);
            return Ok(Expr::Literal(literal_expr));
        }

        match &first_token.token {
            TokenType::Identifier(field_name) => path::parse(parser, first_token, None, field_name),
            TokenType::For => r#for::parse(parser, first_token),
            TokenType::If => r#if::parse(parser),
            TokenType::LeftCurly => r#block::parse(parser),
            _ => Err(parser.unexpected_token(first_token, "expression"))
        }
    }

    pub fn parse_expr(parser: &mut Parser, first_token: &Token) -> Result<Expr, ParseErr> {
        let mut expr = Expr::parse_expr_side(parser, first_token)?;

        destructive_loop!({

            let next_token = parser.collector.next();
            if let Some(operation) = next_token.token.to_operation() {

                expr = binary::parse(parser, first_token, expr, operation)?;

            } else if let Some(assign_op) = next_token.token.to_assign_op() {

                expr = assign::parse(parser, first_token, expr, assign_op)?;

            } else {

                match &next_token.token {
                    TokenType::Dot => {

                        let next_token = parser.collector.next();
                        match &next_token.token {
                            TokenType::Identifier(field_name) => expr = path::parse(parser, next_token, Some(expr), field_name)?,
                            _ => return Err(parser.unexpected_token(next_token, "field"))
                        }

                    },
                    TokenType::LeftParen => expr = call::parse(parser, next_token, expr)?,
                    _ => {
                        parser.collector.back();
                        break
                    }
                }
            }
        });

        Ok(expr)
    }
}

impl Executable for Expr {
    fn exec(&self, interpreter: &mut Interpreter) {
        self.eval(interpreter);
    }
}

impl ParseableStatement for Expr {
    fn parse(parser: &mut Parser, first_token: &Token) -> Result<Statement, ParseErr> {
        let expr = Expr::parse_expr(parser, first_token)?;
        Ok(Statement::Expr(expr))
    }
}

#[cfg(test)]
mod tests;
