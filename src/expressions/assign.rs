use crate::data::Data;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::helpers::destructive_loop;
use crate::interpreter::Interpreter;
use crate::operations::Operation;
use crate::parser::Parser;
use crate::tokenizer::Token;
use crate::types::Type;

#[derive(Debug, Clone)]
pub enum AssignOp {
    Eq,
    AddEq
}

#[derive(Debug, Clone)]
pub struct ExprAssign {
    op: AssignOp,
    path: Box<Expr>,
    value: Box<Expr>
}

impl ExprAssign {
    pub fn new(op: AssignOp, path: Box<Expr>, value: Box<Expr>) -> ExprAssign {
        ExprAssign {
            op,
            path,
            value
        }
    }
}

impl Evaluable for ExprAssign {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let value = self.value.typ(parser)?;
        let old = self.path.typ(parser)?;

        match self.op {
            AssignOp::Eq => self.value.typ(parser),
            AssignOp::AddEq => Operation::Add.typ(&old, &value)
        }
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let value = self.value.eval(interpreter);
        let old = self.path.eval(interpreter);

        let data = match self.op {
            AssignOp::Eq => value,
            AssignOp::AddEq => Operation::Add.op(&old, &value)
        };

        match *self.path {
            Expr::Path(ref path_expr) => {
                interpreter.memory.assign(path_expr.path.to_string(), data.clone());
            },
            _ => panic!("sorry for now you can only assign to direct paths")
        }

        data
    }
}

pub fn parse(parser: &mut Parser, first_token: &Token, expr: Expr) -> Result<Expr, ParseErr> {
    let operation = first_token.token.to_assign_op().unwrap();

    let value_token = parser.collector.next();
    let value = Expr::parse_expr(parser, value_token)?;

    Ok(Expr::Assign(ExprAssign::new(operation, Box::new(expr), Box::new(value))))
}
