use crate::builtin::BuiltinFunc;
use crate::data::Data;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::helpers::destructive_loop;
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
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        if let Some(builtin) = BuiltinFunc::from_name(&self.name) {
            return Ok(builtin.return_type())
        }

        if let Some(Type::Fn { args, return_type }) = parser.sim_memory.get(&self.name) {
            return Ok(**return_type)
        }

        panic!("should have already been caught by the parser")
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let args: Vec<_> = self.args.iter()
            .map(|arg_expr| arg_expr.eval(interpreter))
            .collect();

        if let Some(builtin) = BuiltinFunc::from_name(&self.name) {
            for (i, arg_data) in args.iter().enumerate() {
                let arg_type = &builtin.arg_types()[i];
                interpreter.memory.insert(arg_type.0.to_string(), arg_data.clone());
            }

            return builtin.eval(args);
        }

        if let Some(Data::Fn(func_decl)) = interpreter.memory.get(&self.name).cloned() {
            interpreter.memory.push_scope();

            for (i, arg_data) in args.iter().enumerate() {
                let arg_type = &func_decl.args[i];
                interpreter.memory.insert(arg_type.0.to_string(), arg_data.clone());
            }

            let res = func_decl.body.eval(interpreter);
            interpreter.memory.pop_scope();

            return res;
        }

        panic!("should have already been caught by the parser")
    }
}

pub fn parse(parser: &mut Parser, first_token: &Token, name: &String) -> Result<Expr, ParseErr> {
    let mut args_values = vec![];

    destructive_loop!({
        let next_token = parser.collector.next();
        match &next_token.token {
            TokenType::RightParen => break,
            _ => ()
        }

        // TODO: prob explain to the user that it expects a rightparent too but current error
        // handler doesn't support that so ill fix it in post
        let arg_expr = Expr::parse_expr(parser, next_token)?;

        args_values.push(Box::new(arg_expr));

        let next_token = parser.collector.next();
        match &next_token.token {
            TokenType::Comma => continue,
            TokenType::RightParen => break,
            _ => return Err(parser.unexpected_token(next_token, "Comma or RightParen"))
        }
    });

    if let Some(builtin) = BuiltinFunc::from_name(name) {
        return Ok(Expr::Method(ExprMethod::new(name.to_string(), args_values)));
    }

    if let Some(Type::Fn { args, return_type }) = parser.sim_memory.get(name) {
        return Ok(Expr::Method(ExprMethod::new(name.to_string(), args_values)));
    }

    Err(ParseErrKind::UnknownField(name.to_string()).to_err(first_token.pos))
}
