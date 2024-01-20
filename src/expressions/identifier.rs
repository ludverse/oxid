use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErr, ParseErrKind};
use crate::expressions::{Expr, Evaluable};
use crate::data::Data;
use crate::operations::Operation;
use crate::tokenizer::Token;
use crate::types::Type;

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
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        parser.sim_memory.get(&self.path[0])
            .ok_or(ParseErrKind::UnknownField(self.path[0].clone()))
            .cloned()
    }

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
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        Ok(Type::Number)
    }

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
    fn get_type(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let value = self.value.get_type(parser)?;
        let old = parser.sim_memory.get(&self.path[0])
            .ok_or(ParseErrKind::UnknownField(self.path[0].clone()))?;

        match self.op {
            AssignOp::Eq => self.value.get_type(parser),
            AssignOp::AddEq => Operation::Add.typ(old, &value)
        }
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        // we cannot fetch the old data before we eval the value since the value can be another
        // ExprAssign in the same variable that will modify the variable
        //
        // im not sure any more and i  tested it both ways and it works idk if im stupid
        let value = self.value.eval(interpreter);
        let old = interpreter.memory.get(&self.path[0]).unwrap();

        let data = match self.op {
            AssignOp::Eq => value,
            AssignOp::AddEq => Operation::Add.op(old, &value)
        };

        interpreter.memory.insert(self.path[0].to_string(), data.clone());

        data
    }
}

pub fn parse(parser: &mut Parser, name: &String) -> Result<Expr, ParseErr> {
    let name_pos = parser.collector.current_pos();

    match parser.collector.next() {
        Token::LeftParen => {
            let next_token = parser.collector.next();
            let arg_expr = Expr::parse_expr(parser, next_token)?;

            match parser.collector.next() {
                Token::RightParen => {
                    if name != "println" {
                        return Err(ParseErrKind::UnknownField(name.to_string()).to_err(name_pos));
                    }
                    Ok(Expr::Method(ExprMethod::new(vec![name.to_string()], vec![Box::new(arg_expr)])))
                },
                _ => Err(parser.unexpected_token("RightParen"))
            }
        },
        token if token.to_assign_op().is_some() => {
            let operation = token.to_assign_op().unwrap();

            let next_token = parser.collector.next();
            let expr = Expr::parse_expr(parser, next_token)?;

            if !parser.sim_memory.contains_key(name) {
                return Err(ParseErrKind::UnknownField(name.to_string()).to_err(name_pos));
            }
            Ok(Expr::Assign(ExprAssign::new(operation, vec![name.to_string()], Box::new(expr))))
        },
        _ => {
            parser.collector.back();

            if !parser.sim_memory.contains_key(name) {
                return Err(ParseErrKind::UnknownField(name.to_string()).to_err(name_pos));
            }
            Ok(Expr::Path(ExprPath::new(vec![name.to_string()])))
        }
    }
}

