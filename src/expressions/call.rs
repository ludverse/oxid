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
        let mangled_path = self.path.mangle_path()?;

        let args: Vec<_> = self.args.iter()
            .map(|arg_expr| arg_expr.typ(parser).unwrap()) // IDK how to not unwrap this and error
                                                           // handle this
            .collect();

        if let Some(builtin) = BuiltinFunc::from_name(&mangled_path) {
            return Ok(builtin.type_check(args)?)
        }

        if let Type::Fn { args_types, return_type } = self.path.typ(parser)? {
            // TODO: check for that args types and args have same type

            return Ok(*return_type)
        }

        unreachable!();
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let mangled_path = self.path.mangle_path().unwrap();

        let args: Vec<_> = self.args.iter()
            .map(|arg_expr| arg_expr.eval(interpreter))
            .collect();

        if let Some(builtin) = BuiltinFunc::from_name(&mangled_path) {
            return builtin.eval(args);
        }

        if let Data::Fn(fn_decl) = self.path.eval(interpreter) {
            interpreter.memory.push_scope();

            for (i, arg_data) in args.iter().enumerate() {
                let arg_type = &fn_decl.args[i];
                interpreter.memory.insert(arg_type.0.to_string(), arg_data.clone());
            }

            let res = fn_decl.body.eval(interpreter);
            interpreter.memory.pop_scope();

            return res;
        }

        panic!("should have already been caught by the parser")
    }
}
