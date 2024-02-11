use crate::data::Data;
use crate::errors::ParseErr;
use crate::expressions::block::ExprBlock;
use crate::helpers::destructive_loop;
use crate::interpreter::Interpreter;
use crate::tokenizer::{Token, TokenType};
use crate::parser::Parser;
use crate::statements::{Executable, ParseableStatement, Statement};
use crate::types::Type;

use super::r#fn::FunctionDeclaration;

#[derive(Debug, Clone)]
pub struct ImplStatement {
    typ: Type,
    implementations: Vec<FunctionDeclaration>
}

impl ImplStatement {
    fn new(typ: Type, implementations: Vec<FunctionDeclaration>) -> Self {
        Self {
            typ,
            implementations
        }
    }
}

impl Executable for ImplStatement {
    fn exec(&self, interpreter: &mut Interpreter) {
    }
}

impl ParseableStatement for ImplStatement {
    fn parse(parser: &mut Parser, _first_token: &Token) -> Result<Statement, ParseErr> {

        let next_token = parser.collector.next();
        match &next_token.token {
            TokenType::Identifier(type_name) => {

                let next_token = parser.collector.next();
                match next_token.token {
                    TokenType::LeftParen => {

                        Ok(Statement::FunctionDeclaration(func_decl))
                    }
                    _ => Err(parser.unexpected_token(next_token, "LeftParen"))
                }

            },
            _ => Err(parser.unexpected_token(next_token, "function name"))
        }

    }
}

fn parse_args(parser: &mut Parser) -> Result<Vec<(String, Type)>, ParseErr> {
    let mut args = vec![];

    destructive_loop!({
        let next_token = parser.collector.next();
        match &next_token.token {
            TokenType::Identifier(arg_name) => {

                let next_token = parser.collector.next();
                match &next_token.token {
                    TokenType::Colon => {

                        let next_token = parser.collector.next();
                        match &next_token.token {
                            TokenType::Identifier(arg_type) if Type::from_name(arg_type).is_some() => {

                                let arg_type = Type::from_name(arg_type).unwrap();
                                args.push((arg_name.to_string(), arg_type));

                                let next_token = parser.collector.next();
                                match next_token.token {
                                    TokenType::Comma => (),
                                    TokenType::RightParen => break,
                                    _ => return Err(parser.unexpected_token(next_token, "Comma or RightParen"))
                                }

                            },
                            _ => return Err(parser.unexpected_token(next_token, "type"))
                        }

                    },
                    _ => return Err(parser.unexpected_token(next_token, "Colon"))
                }

            },
            TokenType::RightParen => break,
            _ => return Err(parser.unexpected_token(next_token, "argument or RightParen"))
        }

    });

    Ok(args)
}
