use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind, map_err_token};
use crate::statements::{Executable, ParseableStatement, Statement};
use crate::tokenizer::{Token, TokenType};
use crate::data::{Data, ExprLiteral, ExprBinary};
use crate::types::Type;

use paths::{ExprPath, ExprMethod, ExprAssign};
use r#for::ExprFor;
use r#if::ExprIf;
use bang::ExprUnary;
use block::ExprBlock;

pub mod paths;
pub mod r#for;
pub mod r#if;
pub mod bang;
pub mod block;

pub trait Evaluable {
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind>;
    fn eval(&self, interpreter: &mut Interpreter) -> Data;
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(ExprLiteral),
    Binary(ExprBinary),
    Unary(ExprUnary),
    Path(ExprPath),
    Method(ExprMethod),
    Assign(ExprAssign),
    Block(ExprBlock),
    For(ExprFor),
    If(ExprIf)
}

impl Expr {
    pub fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        match self {
            Expr::Literal(literal_expr) => literal_expr.get_type(parser),
            Expr::Binary(binary_expr) => binary_expr.get_type(parser),
            Expr::Unary(_) => unimplemented!(),
            Expr::Path(path_expr) => path_expr.get_type(parser),
            Expr::Method(expr_method) => expr_method.get_type(parser),
            Expr::Assign(assign_expr) => assign_expr.get_type(parser),
            Expr::Block(block_expr) => block_expr.get_type(parser),
            Expr::For(for_expr) => for_expr.get_type(parser),
            Expr::If(if_expr) => if_expr.get_type(parser)
        }
    }

    pub fn eval(&self, interpreter: &mut Interpreter) -> Data {
        match self {
            Expr::Literal(literal_expr) => literal_expr.eval(interpreter),
            Expr::Binary(binary_expr) => binary_expr.eval(interpreter),
            Expr::Unary(_) => unimplemented!(),
            Expr::Path(path_expr) => path_expr.eval(interpreter),
            Expr::Method(expr_method) => expr_method.eval(interpreter),
            Expr::Assign(assign_expr) => assign_expr.eval(interpreter),
            Expr::Block(block_expr) => block_expr.eval(interpreter),
            Expr::For(for_expr) => for_expr.eval(interpreter),
            Expr::If(if_expr) => if_expr.eval(interpreter)
        }
    }

    fn parse_token_expr(parser: &mut Parser, first_token: &Token) -> Result<Expr, ParseErr> {
        if let Some(data) = first_token.token.to_data() {
            let literal_expr = ExprLiteral::new(data);
            return Ok(Expr::Literal(literal_expr));
        }
        
        match first_token.token {
            TokenType::Identifier(_) => paths::parse(parser, first_token),
            TokenType::For => r#for::parse(parser, first_token),
            TokenType::If => r#if::parse(parser),
            TokenType::LeftCurly => r#block::parse(parser),
            _ => Err(parser.unexpected_token(first_token, "expression"))
        }
    }

    pub fn parse_expr(parser: &mut Parser, first_token: &Token) -> Result<Expr, ParseErr> {
        let mut lhs_pos = first_token.pos;
        let mut lhs = Box::new(Expr::parse_token_expr(parser, first_token)?);
        let mut lhs_type = map_err_token(lhs.get_type(parser), first_token)?;

        for i in 0..=1_000_000 {
            let operation = parser.collector.next();
            match operation.token.to_operation() {
                Some(operation) => {
                    let rhs_token = parser.collector.next();
                    let rhs = Box::new(Expr::parse_token_expr(parser, rhs_token)?);
                    let rhs_type = map_err_token(rhs.get_type(parser), rhs_token)?;

                    operation.typ(&lhs_type, &rhs_type)
                        .map_err(|err_kind| err_kind.to_err(lhs_pos))?;

                    lhs_pos = rhs_token.pos;
                    lhs = Box::new(Expr::Binary(ExprBinary::new(operation, lhs, rhs)));
                    lhs_type = lhs.get_type(parser)
                        .map_err(|err_kind| err_kind.to_err(lhs_pos))?;
                },
                None => {
                    parser.collector.back();
                    return Ok(*lhs)
                }
            }
        }

        panic!("loop never breaked");
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
