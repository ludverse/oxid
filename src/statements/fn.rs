use crate::data::Data;
use crate::errors::ParseErr;
use crate::expressions::block::ExprBlock;
use crate::helpers::destructive_loop;
use crate::interpreter::Interpreter;
use crate::tokenizer::{Token, TokenType};
use crate::parser::Parser;
use crate::statements::{Executable, ParseableStatement, Statement};
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub return_type: Box<Type>,
    pub body: ExprBlock
}

impl FunctionDeclaration {
    fn new(name: String, args: Vec<(String, Type)>, return_type: Box<Type>, body: ExprBlock) -> FunctionDeclaration {
        FunctionDeclaration {
            name,
            args,
            return_type,
            body
        }
    }
}

impl Executable for FunctionDeclaration {
    fn exec(&self, interpreter: &mut Interpreter) {
        interpreter.memory.insert(self.name.to_string(), Data::Fn(self.clone()));
    }
}

impl ParseableStatement for FunctionDeclaration {
    fn parse(parser: &mut Parser, _first_token: &Token) -> Result<Statement, ParseErr> {

        let next_token = parser.collector.next();
        match &next_token.token {
            TokenType::Identifier(name) => {

                let next_token = parser.collector.next();
                match next_token.token {
                    TokenType::LeftParen => {

                        let args = parse_args(parser)?;

                        parser.sim_memory.push_scope();
                        for arg in args.iter() {
                            parser.sim_memory.insert(arg.0.to_string(), arg.1.clone());
                        }

                        let first_token = parser.collector.next();
                        let body = ExprBlock::parse_block(parser, first_token)?;
                        parser.sim_memory.pop_scope();

                        let fn_decl = FunctionDeclaration::new(name.to_string(), args, Box::new(Type::Bool), body);
                        let fn_type = Type::Fn { args: fn_decl.args.clone(), return_type: fn_decl.return_type.clone() };

                        parser.sim_memory.insert(name.to_string(), fn_type);
                        Ok(Statement::FunctionDeclaration(fn_decl))

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
