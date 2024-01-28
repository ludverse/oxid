use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expressions::{Expr, Evaluable, ExprBlock};
use crate::data::{Data, ExprLiteral};
use crate::statements::Statement;
use crate::tokenizer::Token;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct ExprIf {
    pub condition_expr: Box<Expr>,
    pub body: ExprBlock
}

impl ExprIf {
    pub fn new(condition_expr: Box<Expr>, body: ExprBlock) -> ExprIf {
        ExprIf {
            condition_expr,
            body
        }
    }
}

impl Evaluable for ExprIf {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        // self.body.get_type(parser)
        //
        // when we implement else clauses we can start returning values

        Ok(Type::TempNil)
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let condition_expr = self.condition_expr.eval(interpreter);

        match condition_expr {
            Data::Bool(val) => {
                if val {
                    self.body.eval(interpreter);
                }
                Data::TempNil
            },
            _ => panic!("temp")
        }
    }
}

pub fn parse(parser: &mut Parser) -> Result<Expr, ParseErr> {
    let next_token = parser.collector.next();
    let condition_expr = Expr::parse_expr(parser, next_token)?;

    let next_token = parser.collector.next();
    let body = ExprBlock::parse_block(parser, next_token)?;

    let if_expr = ExprIf::new(Box::new(condition_expr), body);
    Ok(Expr::If(if_expr))
}
