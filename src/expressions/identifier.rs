use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expressions::{Expr, Evaluable};
use crate::expressions::data::Data;
use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct ExprPath {
    path: Vec<String>
}

impl ExprPath {
    pub fn new(path: Vec<String>) -> ExprPath {
        ExprPath {
            path
        }
    }
}

impl Evaluable for ExprPath {
    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        interpreter.memory.get(&self.path[0]).unwrap().clone()
    }
}

#[derive(Debug, Clone)]
pub struct ExprMethod {
    path: Vec<String>,
    args: Vec<Box<Expr>>
}

impl ExprMethod {
    pub fn new(path: Vec<String>, args: Vec<Box<Expr>>) -> ExprMethod {
        ExprMethod {
            path,
            args
        }
    }
}

impl Evaluable for ExprMethod {
    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let data = self.args[0].eval(interpreter);
        match data {
            Data::String(val) => println!("{}", val),
            _ => println!("{:?}", data)
        }

        Data::Number(0.)
    }
}

#[derive(Debug, Clone)]
pub struct ExprAssign {
    path: Vec<String>,
    value: Box<Expr>
}

impl ExprAssign {
    pub fn new(path: Vec<String>, value: Box<Expr>) -> ExprAssign {
        ExprAssign {
            path,
            value
        }
    }
}

impl Evaluable for ExprAssign {
    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let data = self.value.eval(interpreter);
        interpreter.memory.insert(self.path[0].to_string(), data.clone());

        data
    }
}

pub fn parse(parser: &mut Parser, name: &String) -> Result<Expr, ParseErr> {
    match parser.collector.next() {
        Token::Equal => {
            let expr_token = parser.collector.next();
            let expr = parser.parse_expr(expr_token)?;

            if !parser.sim_memory.contains_key(name) {
                return Err(ParseErrKind::UnknownField(name.to_string()).to_err(parser.collector.current_pos()));
            }
            Ok(Expr::Assign(ExprAssign::new(vec![name.to_string()], Box::new(expr))))
        },
        Token::LeftParen => {
            let expr_token = parser.collector.next();
            let arg_expr = parser.parse_expr(expr_token)?;

            match parser.collector.next() {
                Token::RightParen => {
                    if name != "println" {
                        return Err(ParseErrKind::UnknownField(name.to_string()).to_err(parser.collector.current_pos()));
                    }
                    Ok(Expr::Method(ExprMethod::new(vec![name.to_string()], vec![Box::new(arg_expr)])))
                },
                _ => Err(parser.unexpected_token("RightParen"))
            }
        },
        _ => {
            parser.collector.back();

            if !parser.sim_memory.contains_key(name) {
                return Err(ParseErrKind::UnknownField(name.to_string()).to_err(parser.collector.current_pos()));
            }
            Ok(Expr::Path(ExprPath::new(vec![name.to_string()])))
        }
    }
}

