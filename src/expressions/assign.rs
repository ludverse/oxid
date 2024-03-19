use crate::data::Data;
use crate::errors::{ParseErrKind, ParseErr, map_err_token};
use crate::expressions::{Expr, Evaluable};
use crate::interpreter::Interpreter;
use crate::operations::Operation;
use crate::parser::Parser;
use crate::tokenizer::token::Token;
use crate::types::Type;

#[derive(Debug, Clone)]
pub enum AssignOp {
    Eq,
    AddEq
}

#[derive(Debug, Clone)]
pub struct ExprAssign {
    op: AssignOp,
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl ExprAssign {
    pub fn new(op: AssignOp, lhs: Box<Expr>, rhs: Box<Expr>) -> ExprAssign {
        ExprAssign {
            op,
            lhs,
            rhs
        }
    }
}

impl Evaluable for ExprAssign {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let mangled = self.lhs.mangle_path().unwrap();

        let value = self.rhs.typ(parser).unwrap();
        let old = parser.sim_memory.get(&mangled).unwrap();

        let res = match self.op {
            AssignOp::Eq => self.rhs.typ(parser).unwrap(),
            AssignOp::AddEq => Operation::Add.typ(old, &value).unwrap()
        };

        Ok(res)
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let mangled = self.lhs.mangle_path().unwrap();

        let value = self.rhs.eval(interpreter);
        let old = interpreter.memory.get(&mangled).unwrap();

        let data = match self.op {
            AssignOp::Eq => value,
            AssignOp::AddEq => Operation::Add.op(old, &value)
        };

        interpreter.memory.assign(mangled, data.clone());

        data
    }
}

pub fn parse(parser: &mut Parser, first_token: &Token, expr: Expr, assign_op: AssignOp) -> Result<Expr, ParseErr> {
    map_err_token(expr.mangle_path(), first_token)?;

    let rhs_token = parser.collector.next();
    let rhs = Expr::parse_expr(parser, rhs_token)?;

    let expr_assign = ExprAssign::new(assign_op, Box::new(expr), Box::new(rhs));

    Ok(Expr::Assign(expr_assign))
}
