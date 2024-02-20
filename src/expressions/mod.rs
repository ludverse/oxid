use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind, map_err_token};
use crate::statements::{Executable, ParseableStatement, Statement};
use crate::tokenizer::{Token, TokenType};
use crate::data::{Data, ExprLiteral, ExprBinary};
use crate::types::Type;
use crate::helpers::destructive_loop;

use index::ExprIndex;
use bang::ExprUnary;
use path::ExprPath;
use assign::ExprAssign;
use block::ExprBlock;
use r#for::ExprFor;
use r#if::ExprIf;

pub mod index;
pub mod bang;
pub mod path;
pub mod assign;
pub mod block;
pub mod r#for;
pub mod r#if;

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
    Method(),
    Assign(ExprAssign),
    Block(ExprBlock),
    For(ExprFor),
    If(ExprIf)
}

impl Expr {
    pub fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        match self {
            Expr::Literal(literal_expr) => literal_expr.typ(parser),
            Expr::Binary(binary_expr) => binary_expr.typ(parser),
            Expr::Index(index_expr) => index_expr.typ(parser),
            Expr::Unary(_) => unimplemented!(),
            Expr::Path(path_expr) => path_expr.typ(parser),
            Expr::Method() => unimplemented!(), //expr_method.typ(parser),
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
            Expr::Method() => unimplemented!(),
            // Expr::Method(expr_method) => expr_method.eval(interpreter),
            Expr::Assign(assign_expr) => assign_expr.eval(interpreter),
            Expr::Block(block_expr) => block_expr.eval(interpreter),
            Expr::For(for_expr) => for_expr.eval(interpreter),
            Expr::If(if_expr) => if_expr.eval(interpreter)
        }
    }

    fn parse_expr_side(parser: &mut Parser, first_token: &Token) -> Result<Expr, ParseErr> {
        if let Some(data) = first_token.token.to_data() {
            let literal_expr = ExprLiteral::new(data);
            return Ok(Expr::Literal(literal_expr));
        }

        match first_token.token {
            TokenType::Identifier(_) => path::parse(parser, first_token),
            TokenType::For => r#for::parse(parser, first_token),
            TokenType::If => r#if::parse(parser),
            TokenType::LeftCurly => r#block::parse(parser),
            _ => Err(parser.unexpected_token(first_token, "expression"))
        }
    }

    pub fn parse_expr(parser: &mut Parser, first_token: &Token) -> Result<Expr, ParseErr> {
        let mut expr_pos = first_token.pos;
        let mut expr = Expr::parse_expr_side(parser, first_token)?;
        let mut expr_type = map_err_token(expr.typ(parser), first_token)?;

        destructive_loop!({

            let next_token = parser.collector.next();
            if let Some(operation) = next_token.token.to_operation() {

                let rhs_token = parser.collector.next();
                let rhs = Box::new(Expr::parse_expr_side(parser, rhs_token)?);
                let rhs_type = map_err_token(rhs.typ(parser), rhs_token)?;

                operation.typ(&expr_type, &rhs_type)
                    .map_err(|err_kind| err_kind.to_err(expr_pos))?;

                expr_pos = rhs_token.pos;
                expr = Expr::Binary(ExprBinary::new(operation, Box::new(expr), rhs));
                expr_type = expr.typ(parser)
                    .map_err(|err_kind| err_kind.to_err(expr_pos))?;

            } else if let Some(assign_op) = next_token.token.to_assign_op() {

                match expr {
                    Expr::Path(path_expr) => {

                        let rhs_token = parser.collector.next();
                        let rhs = Expr::parse_expr(parser, rhs_token)?;

                        expr = Expr::Assign(ExprAssign::new(assign_op, path_expr.field, Box::new(rhs)));

                    },
                    _ => return Err(ParseErrKind::InvalidLeftHandsideOfAssignment(format!("{:?}", expr)).to_err(expr_pos))
                }

            } else {
                match &next_token.token {
                    TokenType::LeftBrace => {
                        // indexing
                    },
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
