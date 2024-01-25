use crate::data::Data;
use crate::errors::ParseErr;
use crate::expressions::block::ExprBlock;
use crate::helpers::destructive_loop;
use crate::interpreter::Interpreter;
use crate::tokenizer::{Token, TokenType};
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

        let next_token = parser.collector.next();
        match &next_token.token {
            TokenType::Identifier(name) => {

                let next_token = parser.collector.next();
                match next_token.token {
                    TokenType::LeftParen => {

                        let args = parse_args(parser)?;

                        for arg in args.iter() {
                            parser.sim_memory.insert(arg.0.to_string(), arg.1.clone());
                        }

                        let first_token = parser.collector.next();
                        let body = ExprBlock::parse_block(parser, first_token)?;

                        let func_decl = FunctionDeclaration::new(name.to_string(), args, Type::Bool, body);

                        parser.functions.insert(name.to_string(), func_decl.signature.clone());

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
