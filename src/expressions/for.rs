use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expressions::{Expr, Evaluable};
use crate::expressions::data::Data;
use crate::statements::Statement;
use crate::tokenizer::Token;

use super::data::ExprLiteral;

#[derive(Debug, Clone)]
pub struct ExprFor {
    pub start_i: Box<Expr>,
    pub end_i: Box<Expr>,
    pub index_var: String,
    pub body: Vec<Statement>
}

impl ExprFor {
    pub fn new(start_i: Box<Expr>, end_i: Box<Expr>, index_var: String, body: Vec<Statement>) -> ExprFor {
        ExprFor {
            start_i,
            end_i,
            index_var,
            body
        }
    }
}

impl Evaluable for ExprFor {
    fn eval(&self, interpreter: &mut crate::interpreter::Interpreter) -> Data {
        let start_i = self.start_i.eval(interpreter);
        let end_i = self.end_i.eval(interpreter);

        match start_i {
            Data::Number(start_i) => {
                match end_i {
                    Data::Number(end_i) => {
                        for i in start_i as usize..end_i as usize {
                            interpreter.memory.insert(self.index_var.to_string(), Data::Number(i as f64));
                            interpreter.interpret_block(&self.body);
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        Data::Number(0.)
    }
}

pub fn parse(parser: &mut Parser) -> Result<Expr, ParseErr> {
    match parser.collector.next() {
        Token::Identifier(index_var) => match parser.collector.next() {
            Token::In => {
                let next_token = parser.collector.next();
                let start_expr = parser.parse_expr(next_token)?;

                match parser.collector.next() {
                    Token::Range => {
                        let next_token = parser.collector.next();
                        let end_expr = parser.parse_expr(next_token)?;

                        match parser.collector.next() {
                            Token::LeftCurly => {
                                let i_val = Expr::Literal(ExprLiteral::new(Data::Number(0.)));
                                parser.sim_memory.insert(index_var.to_string(), i_val);

                                let body = parser.parse_block()?;
                                let for_expr = ExprFor::new(Box::new(start_expr), Box::new(end_expr), index_var.to_string(), body);

                                Ok(Expr::For(for_expr))
                            },
                            _ => Err(parser.unexpected_token("LeftCurly"))
                        }
                    },
                    _ => Err(parser.unexpected_token("Range"))
                }
            },
            _ => Err(parser.unexpected_token("In"))
        },
        _ => Err(parser.unexpected_token("variable name"))
    }
}

