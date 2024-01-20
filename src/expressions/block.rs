use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expressions::{Expr, Evaluable};
use crate::data::{Data, ExprLiteral};
use crate::statements::Statement;
use crate::tokenizer::Token;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct ExprBlock {
    pub body: Vec<Statement>
}

impl ExprBlock {
    pub fn new(body: Vec<Statement>) -> ExprBlock {
        ExprBlock {
            body
        }
    }

    fn parse_block_end(parser: &mut Parser, first_token: &Token) -> Result<ExprBlock, ParseErr> {
        let mut body = vec![];

        for i in 0..=1_000_000 {
            if i == 1_000_000 { panic!("loop never breaked") }

            let next_token = parser.collector.next();
            match next_token {
                Token::RightCurly => break,
                _ => body.push(Statement::parse(parser, next_token)?)
            }
        }

        Ok(ExprBlock::new(body))
    }

    pub fn parse_block(parser: &mut Parser, first_token: &Token) -> Result<ExprBlock, ParseErr> {
        match first_token {
            Token::LeftCurly => Self::parse_block_end(parser, first_token),
            _ => Err(parser.unexpected_token("LeftCurly"))
        }
    }
}

impl Evaluable for ExprBlock {
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let last_statement = self.body.last();

        match last_statement {
            Some(Statement::Expr(expr)) => Ok(expr.get_type(parser)?),
            _ => Ok(Type::TempNil)
        }
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let mut res = None;

        for (i, statement) in self.body.iter().enumerate() {
            match statement {
                Statement::Expr(expr) => {
                    if i == self.body.len() - 1 {
                        res = Some(expr.eval(interpreter));
                    } else {
                        expr.eval(interpreter);
                    }
                },
                _ => statement.exec(interpreter)
            }
        }

        res.unwrap_or(Data::TempNil)
    }
}

pub fn parse(parser: &mut Parser) -> Result<Expr, ParseErr> {
    let first_token = parser.collector.next();
    ExprBlock::parse_block_end(parser, first_token)
        .map(|block_expr| Expr::Block(block_expr))
}
