use crate::data::Data;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::helpers::destructive_loop;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::{token::Token, token_type::TokenType};
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct ExprCall {
    path: Box<Expr>,
    args: Vec<Box<Expr>>
}

impl ExprCall {
    pub fn new(path: Box<Expr>, args: Vec<Box<Expr>>) -> Self {
        Self {
            path,
            args
        }
    }
}

impl Evaluable for ExprCall {
    fn typ(&self, parser: &Parser) -> Type {
        let _mangled_path = self.path.mangle_path().unwrap();

        let fn_type = self.path.typ(parser);
        let args: Vec<_> = self.args.iter()
            .map(|arg_expr| arg_expr.typ(parser))
            .collect();

        if let Type::Fn { args_types: _, return_type } = fn_type {
            // TODO: check for that args types and args have same type

            return *return_type
        }

        if let Type::BuiltinFn(builtin) = fn_type {
            return builtin.type_check(args);
        }

        unreachable!("should have already been caught in parsing");
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let _mangled_path = self.path.mangle_path().unwrap();

        let fn_data = self.path.eval(interpreter);
        let args: Vec<_> = self.args.iter()
            .map(|arg_expr| arg_expr.eval(interpreter))
            .collect();

        if let Data::Fn(fn_decl) = fn_data {
            interpreter.memory.push_scope();

            for (i, arg_data) in args.iter().enumerate() {
                let arg_type = &fn_decl.args[i];
                interpreter.memory.insert(arg_type.0.to_string(), arg_data.clone());
            }

            let res = fn_decl.body.eval(interpreter);
            interpreter.memory.pop_scope();

            return res;
        }

        if let Data::BuiltinFn(builtin_fn) = fn_data {
            return builtin_fn.eval(args);
        }

        unreachable!();
    }
}

pub fn parse(parser: &mut Parser, _first_token: &Token, expr: Expr) -> Result<Expr, ParseErr> {
    let mut args = vec![];

    destructive_loop!({
        let next_token = parser.collector.next();
        match &next_token.token {
            TokenType::RightParen => break,
            _ => ()
        }

        // TODO: prob explain to the user that it expects a rightparent too but current error
        // handler doesn't support that so ill fix it in post
        let arg_expr = Expr::parse_expr(parser, next_token)?;

        args.push(Box::new(arg_expr));

        let next_token = parser.collector.next();
        match &next_token.token {
            TokenType::Comma => continue,
            TokenType::RightParen => break,
            _ => return Err(parser.unexpected_token(next_token, "Comma or RightParen"))
        }
    });

    let expr_type = expr.typ(parser);

    match expr_type {
        Type::Fn { args_types: _, return_type: _ } => (),
        Type::BuiltinFn(_) => (),
        _ => return Err(ParseErrKind::NotCallable().from_token(_first_token))
    }

    Ok(Expr::Call(ExprCall::new(Box::new(expr), args)))
}
