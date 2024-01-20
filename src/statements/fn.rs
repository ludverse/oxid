use crate::errors::ParseErr;
use crate::expressions::ExprBlock;
use crate::interpreter::Interpreter;
use crate::tokenizer::Token;
use crate::parser::Parser;
use crate::statements::{Executable, ParseableStatement, Statement};
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: ExprBlock
}

impl FunctionDeclaration {
    fn new(name: String, args: Vec<(String, Type)>, return_type: Type, body: ExprBlock) -> FunctionDeclaration {
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

    }
}

impl ParseableStatement for FunctionDeclaration {
    fn parse(parser: &mut Parser, _first_token: &Token) -> Result<Statement, ParseErr> {
        match parser.collector.next() {
            Token::Identifier(name) => match parser.collector.next() {
                Token::LeftParen => {
                    let mut args = vec![];

                    for _ in 0..=1_000_000 {
                        match parser.collector.next() {
                            Token::Identifier(arg_name) => match parser.collector.next() {
                                Token::Comma => args.push((arg_name.to_string(), Type::Bool)),
                                Token::RightParen => {
                                    args.push((arg_name.to_string(), Type::Bool));

                                    break;
                                },
                                _ => return Err(parser.unexpected_token("Comma or RightParen"))
                            },
                            Token::RightParen => break,
                            _ => return Err(parser.unexpected_token("argument or RightParen"))
                        }
                    }

                    for arg in args.iter() {
                        parser.sim_memory.insert(arg.0.to_string(), arg.1.clone());
                    }

                    let first_token = parser.collector.next();
                    let body = ExprBlock::parse_block(parser, first_token)?;

                    let func_decl = FunctionDeclaration::new(name.to_string(), args, Type::Bool, body);
                    Ok(Statement::FunctionDeclaration(func_decl))
                }
                _ => Err(parser.unexpected_token("LeftParen"))
            },
            _ => Err(parser.unexpected_token("function name"))
        }
    }
}
