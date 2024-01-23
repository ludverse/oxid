use crate::builtin::BuiltinFunc;
use crate::data::Data;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::{Token, TokenType};
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct ExprMethod {
    name: String,
    args: Vec<Box<Expr>>
}

impl ExprMethod {
    pub fn new(name: String, args: Vec<Box<Expr>>) -> ExprMethod {
        ExprMethod {
            name,
            args
        }
    }
}

impl Evaluable for ExprMethod {
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        dbg!(&self.name);
        if let Some(builtin) = BuiltinFunc::from_name(&self.name) {
            return Ok(builtin.return_type())
        }

        if let Some(signature) = parser.functions.get(&self.name) {
            return Ok(signature.return_type.clone())
        }

        panic!("should have already been caught by the parser")
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        if let Some(builtin) = BuiltinFunc::from_name(&self.name) {
            let args: Vec<_> = self.args.iter()
                .map(|arg| arg.eval(interpreter))
                .collect();

            return builtin.eval(args);
        }

        if let Some(func_decl) = interpreter.functions.get(&self.name).cloned() {
            return func_decl.body.eval(interpreter)
        }

        panic!("should have already been caught by the parser")
    }
}

pub fn parse(parser: &mut Parser, first_token: &Token, name: &String) -> Result<Expr, ParseErr> {
    let next_token = parser.collector.next();
    let arg_expr = Expr::parse_expr(parser, next_token)?;

    let paren_token = parser.collector.next();
    match paren_token.token {
        TokenType::RightParen => {
            if let Some(builtin) = BuiltinFunc::from_name(name) {
                return Ok(Expr::Method(ExprMethod::new(name.to_string(), vec![Box::new(arg_expr)])));
            }

            if let Some(signature) = parser.functions.get(name) {
                return Ok(Expr::Method(ExprMethod::new(name.to_string(), vec![Box::new(arg_expr)])));
            }

            return Err(ParseErrKind::UnknownField(name.to_string()).to_err(first_token.pos));
        },
        _ => Err(parser.unexpected_token(paren_token, "RightParen"))
    }
}
