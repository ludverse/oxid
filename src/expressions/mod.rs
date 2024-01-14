use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expr::{Expr, Operation, Data, Evaluable};
use crate::tokenizer::Token;

pub mod identifier;
pub mod r#for;

#[derive(Debug, Clone)]
pub struct ExprLiteral {
    data: Data
}

impl ExprLiteral {
    pub fn new(data: Data) -> ExprLiteral {
        ExprLiteral {
            data
        }
    }
}

impl Evaluable for ExprLiteral {
    fn eval(&self, interpreter: &mut crate::interpreter::Interpreter) -> Data {
        self.data.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ExprBinary {
    operation: Operation,
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl ExprBinary {
    pub fn new(operation: Operation, lhs: Box<Expr>, rhs: Box<Expr>) -> ExprBinary {
        ExprBinary {
            operation,
            lhs,
            rhs
        }
    }
}

impl Evaluable for ExprBinary {
    fn eval(&self, interpreter: &mut crate::interpreter::Interpreter) -> Data {
        let mut lhs = self.lhs.eval(interpreter);
        lhs.op(self.operation, &self.rhs.eval(interpreter)).unwrap();

        lhs
    }
}

pub fn parse_expr_data(parser: &mut Parser, first_token: &Token) -> Result<Box<Expr>, ParseErr> {
    if let Some(data) = first_token.to_data() {
        let literal_expr = ExprLiteral::new(data);
        return Ok(Box::new(Expr::Literal(literal_expr)));
    }
    
    match first_token {
        Token::Identifier(name) => identifier::parse(parser, name),
        Token::For => r#for::parse(parser),
        _ => Err(parser.unexpected_token("expression"))
    }
}

