use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expressions::{Expr, Evaluable};
use crate::data::{Data, Operation};
use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum AssignOp {
    Eq,
    AddEq
}

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
    op: AssignOp,
    path: Vec<String>,
    value: Box<Expr>
}

impl ExprAssign {
    pub fn new(op: AssignOp, path: Vec<String>, value: Box<Expr>) -> ExprAssign {
        ExprAssign {
            op,
            path,
            value
        }
    }
}

impl Evaluable for ExprAssign {
    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let old = interpreter.memory.remove(&self.path[0]).unwrap();
        let value = self.value.eval(interpreter);

        let data = match self.op {
            AssignOp::Eq => value,
            AssignOp::AddEq => old.op(Operation::Add, &value).unwrap()
        };

        interpreter.memory.insert(self.path[0].to_string(), data.clone());

        data
    }
}

pub fn parse(parser: &mut Parser, name: &String) -> Result<Expr, ParseErr> {
    match parser.collector.next() {
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
        token if token.to_assign_op().is_some() => {
            let operation = token.to_assign_op().unwrap();

            let expr_token = parser.collector.next();
            let expr = parser.parse_expr(expr_token)?;

            if !parser.sim_memory.contains_key(name) {
                return Err(ParseErrKind::UnknownField(name.to_string()).to_err(parser.collector.current_pos()));
            }
            Ok(Expr::Assign(ExprAssign::new(operation, vec![name.to_string()], Box::new(expr))))
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

