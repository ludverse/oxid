use crate::data::Data;
use crate::errors::ParseErr;
use crate::expressions::block::ExprBlock;
use crate::interpreter::Interpreter;
use crate::tokenizer::Token;
use crate::parser::Parser;
use crate::statements::{Executable, ParseableStatement, Statement};
use crate::types::Type;

pub type DataArgs = Vec<Data>;
pub type SignatureArgs = Vec<(String, Type)>;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub args: SignatureArgs,
    pub return_type: Type,
}

impl FunctionSignature {
    fn new(args: Vec<(String, Type)>, return_type: Type) -> FunctionSignature {
        FunctionSignature {
            args,
            return_type
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub signature: FunctionSignature,
    pub body: ExprBlock
}

impl FunctionDeclaration {
    fn new(name: String, args: Vec<(String, Type)>, return_type: Type, body: ExprBlock) -> FunctionDeclaration {
        let signature = FunctionSignature::new(args, return_type);

        FunctionDeclaration {
            name,
            signature,
            body
        }
    }
}

impl Executable for FunctionDeclaration {
    fn exec(&self, interpreter: &mut Interpreter) {
        interpreter.functions.insert(self.name.to_string(), self.clone());
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

                    parser.functions.insert(name.to_string(), func_decl.signature.clone());

                    Ok(Statement::FunctionDeclaration(func_decl))
                }
                _ => Err(parser.unexpected_token("LeftParen"))
            },
            _ => Err(parser.unexpected_token("function name"))
        }
    }
}
