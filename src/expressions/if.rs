use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expressions::{Expr, Evaluable};
use crate::data::{Data, ExprLiteral};
use crate::statements::Statement;
use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct ExprIf {
    pub condition_expr: Box<Expr>,
    pub body: Vec<Statement>
}

impl ExprIf {
    pub fn new(condition_expr: Box<Expr>, body: Vec<Statement>) -> ExprIf {
        ExprIf {
            condition_expr,
            body
        }
    }
}

impl Evaluable for ExprIf {
    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let condition_expr = self.condition_expr.eval(interpreter);

        match condition_expr {
            Data::Bool(val) => {
                if val {
                    interpreter.interpret_block(&self.body);
                }
            },
            _ => panic!("temp")
        }

        Data::Number(0.)
    }
}

pub fn parse(parser: &mut Parser) -> Result<Expr, ParseErr> {
    let next_token = parser.collector.next();
    let condition_expr = parser.parse_expr(next_token)?;

    match parser.collector.next() {
        Token::LeftCurly => {
            let body = parser.parse_block()?;
            let if_expr = ExprIf::new(Box::new(condition_expr), body);

            Ok(Expr::If(if_expr))
        },
        _ => Err(parser.unexpected_token("LeftCurly"))
    }
}

