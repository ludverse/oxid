use crate::data::Data;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::helpers::destructive_loop;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::{Token, TokenType};
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
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        // if let Some(builtin) = BuiltinFunc::from_name(&self.name) {
        //     return Ok(builtin.return_type())
        // }

        
        if let Type::Fn(signature) = self.path.typ(parser)? {
            return Ok(*signature.ret.clone())
        }

        panic!("should have already been caught by the parser")
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let args: Vec<_> = self.args.iter()
            .map(|arg_expr| arg_expr.eval(interpreter))
            .collect();

        // if let Some(builtin) = BuiltinFunc::from_name(&self.name) {
        //     for (i, arg_data) in args.iter().enumerate() {
        //         let arg_type = &builtin.arg_types()[i];
        //         interpreter.memory.insert(arg_type.0.to_string(), arg_data.clone());
        //     }
        //
        //     return builtin.eval(args);
        // }

        if let Data::Fn(func_decl) = self.path.eval(interpreter) {
            interpreter.memory.push_scope();

            for (i, arg_data) in args.iter().enumerate() {
                let arg_type = &func_decl.signature.args[i];
                interpreter.memory.insert(arg_type.0.to_string(), arg_data.clone());
            }

            let res = func_decl.body.eval(interpreter);
            interpreter.memory.pop_scope();

            return res;
        }

        panic!("should have already been caught by the parser")
    }
}

pub fn parse(parser: &mut Parser, first_token: &Token, expr: Expr) -> Result<Expr, ParseErr> {
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

    // if let Some(builtin) = BuiltinFunc::from_name(name) {
    //     return Ok(Expr::Method(ExprCall::new(name.to_string(), args)));
    // }

    Ok(Expr::Method(ExprCall::new(Box::new(expr), args)))
}
