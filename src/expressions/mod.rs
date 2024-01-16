use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::tokenizer::Token;
use crate::data::{Data, ExprLiteral, ExprBinary};
use identifier::{ExprPath, ExprMethod, ExprAssign};
use r#for::ExprFor;
use r#if::ExprIf;
use bang::ExprUnary;

pub mod identifier;
pub mod r#for;
pub mod r#if;
pub mod bang;

pub fn parse_expr_data(parser: &mut Parser, first_token: &Token) -> Result<Expr, ParseErr> {
    if let Some(data) = first_token.to_data() {
        let literal_expr = ExprLiteral::new(data);
        return Ok(Expr::Literal(literal_expr));
    }
    
    match first_token {
        Token::Identifier(name) => identifier::parse(parser, name),
        Token::For => r#for::parse(parser),
        Token::If => r#if::parse(parser),
        _ => Err(parser.unexpected_token("expression"))
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(ExprLiteral),
    Binary(ExprBinary),
    Unary(ExprUnary),
    Path(ExprPath),
    Method(ExprMethod),
    Assign(ExprAssign),
    For(ExprFor),
    If(ExprIf)
}

pub trait Evaluable {
    fn eval(&self, interpreter: &mut Interpreter) -> Data;
}

impl Expr {
    pub fn eval(&self, interpreter: &mut Interpreter) -> Data {
        match self {
            Expr::Literal(literal_expr) => literal_expr.eval(interpreter),
            Expr::Binary(binary_expr) => binary_expr.eval(interpreter),
            Expr::Unary(_) => unimplemented!(),
            Expr::Path(path_expr) => path_expr.eval(interpreter),
            Expr::Method(expr_method) => expr_method.eval(interpreter),
            Expr::Assign(assign_expr) => assign_expr.eval(interpreter),
            Expr::For(for_expr) => for_expr.eval(interpreter),
            Expr::If(if_expr) => if_expr.eval(interpreter)
        }
    }
}

