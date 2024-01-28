use crate::expressions::index::ExprIndex;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind, map_err_token};
use crate::statements::{Executable, ParseableStatement, Statement};
use crate::tokenizer::{Token, TokenType};
use crate::data::{Data, ExprLiteral, ExprBinary};
use crate::types::Type;
use crate::helpers::destructive_loop;

use paths::{ExprPath, ExprMethod, ExprAssign};
use r#for::ExprFor;
use r#if::ExprIf;
use bang::ExprUnary;
use block::ExprBlock;

pub mod paths;
pub mod r#for;
pub mod r#if;
pub mod bang;
pub mod index;
pub mod block;

pub trait Evaluable {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind>;
    fn eval(&self, interpreter: &mut Interpreter) -> Data;
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(ExprLiteral),
    Binary(ExprBinary),
    Index(ExprIndex),
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
            Expr::Literal(literal_expr) => literal_expr.typ(parser),
            Expr::Binary(binary_expr) => binary_expr.typ(parser),
            Expr::Index(index_expr) => index_expr.typ(parser),
            Expr::Unary(_) => unimplemented!(),
            Expr::Path(path_expr) => path_expr.typ(parser),
            Expr::Method(expr_method) => expr_method.typ(parser),
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

        destructive_loop!({

            let next_token = parser.collector.next();
            if let Some(operation) = next_token.token.to_operation() {
                let rhs_token = parser.collector.next();
                let rhs = Box::new(Expr::parse_token_expr(parser, rhs_token)?);
                let rhs_type = map_err_token(rhs.get_type(parser), rhs_token)?;

                operation.typ(&lhs_type, &rhs_type)
                    .map_err(|err_kind| err_kind.to_err(lhs_pos))?;

                lhs_pos = rhs_token.pos;
                lhs = Box::new(Expr::Binary(ExprBinary::new(operation, lhs, rhs)));
                lhs_type = lhs.get_type(parser)
                    .map_err(|err_kind| err_kind.to_err(lhs_pos))?;
            } else {
                match &next_token.token {
                    TokenType::LeftBrace => {
                        let index_token = parser.collector.next();
                        let index_expr = Expr::parse_expr(parser, index_token)?;
                        let expr_type = map_err_token(index_expr.get_type(parser), index_token)?;

                        let next_token = parser.collector.next();
                        match &next_token.token {
                            TokenType::RightBrace => {
                                lhs_pos = index_token.pos;
                                lhs = Box::new(Expr::Index(ExprIndex::new(Box::new(index_expr), lhs)));
                                lhs_type = lhs.get_type(parser)
                                    .map_err(|err_kind| err_kind.to_err(lhs_pos))?;
                            },
                            _ => return Err(parser.unexpected_token(next_token, "RightBracket"))
                        }
                    },
                    _ => {
                        parser.collector.back();
                        return Ok(*lhs)
                    }
                }
            }
        });

        unreachable!()
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
