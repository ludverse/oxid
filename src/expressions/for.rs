use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expressions::{Expr, Evaluable, ExprBlock};
use crate::data::Data;
use crate::tokenizer::{token::Token, token_type::TokenType};
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct ExprFor {
    pub start_i: Box<Expr>,
    pub end_i: Box<Expr>,
    pub index_var: String,
    pub body: ExprBlock
}

impl ExprFor {
    pub fn new(start_i: Box<Expr>, end_i: Box<Expr>, index_var: String, body: ExprBlock) -> ExprFor {
        ExprFor {
            start_i,
            end_i,
            index_var,
            body
        }
    }
}

impl Evaluable for ExprFor {
    fn type_check(&self, _parser: &Parser) -> Type {
        Type::TempNil
    }


    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let start_i = self.start_i.eval(interpreter);
        let end_i = self.end_i.eval(interpreter);

        match start_i {
            Data::Number(start_i) => {
                match end_i {
                    Data::Number(end_i) => {
                        for i in start_i as usize..end_i as usize {
                            interpreter.memory.push_scope();
                            interpreter.memory.insert(self.index_var.to_string(), Data::Number(i as f64));

                            self.body.eval(interpreter);
                            interpreter.memory.pop_scope();
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }

        Data::TempNil
    }
}

pub fn parse(parser: &mut Parser, _first_token: &Token) -> Result<Expr, ParseErr> {
    let next_token = parser.collector.next();
    match &next_token.token {
        TokenType::Identifier(index_var) => {

            let next_token = parser.collector.next();
            match next_token.token {
                TokenType::In => {

                    let next_token = parser.collector.next();
                    let start_expr = Expr::parse_expr(parser, next_token)?;

                    let next_token = parser.collector.next();
                    match next_token.token {
                        TokenType::Spread => {

                            let next_token = parser.collector.next();
                            let end_expr = Expr::parse_expr(parser, next_token)?;

                            parser.sim_memory.push_scope();
                            parser.sim_memory.insert(index_var.to_string(), Type::Number);

                            let next_token = parser.collector.next();
                            let body = ExprBlock::parse_block(parser, next_token)?;
                            parser.sim_memory.pop_scope();

                            let for_expr = ExprFor::new(Box::new(start_expr), Box::new(end_expr), index_var.to_string(), body);
                            Ok(Expr::For(for_expr))

                        },
                        _ => Err(parser.unexpected_token(next_token, "Range"))
                    }

                },
                _ => Err(parser.unexpected_token(next_token, "In"))
            }

        },
        _ => Err(parser.unexpected_token(next_token, "variable name"))
    }
}

