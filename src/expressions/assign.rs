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
    fn type_check(&self, parser: &Parser) -> Type {
        let mangled = self.lhs.mangle_path().unwrap();

        let value = self.rhs.type_check(parser);
        let old = parser.sim_memory.get(&mangled).unwrap();

        match self.op {
            AssignOp::Eq => self.rhs.type_check(parser),
            AssignOp::AddEq => Operation::Add.typ(old, &value).unwrap()
        }
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
    // check if lhs is able to be used as a path
    expr.mangle_path()
        .ok_or_else(||
            ParseErrKind::InvalidPathUse(format!("{:?}", expr))
            .from_token(first_token)
        )?;

    let expr_type = expr.type_check(parser);

    let rhs_token = parser.collector.next();
    let rhs = Expr::parse_expr(parser, rhs_token)?;
    let rhs_type = rhs.type_check(parser);

    let op_is_valid = match assign_op {
        AssignOp::AddEq => Operation::Add.typ(&expr_type, &rhs_type).is_some(),
        _ => true // TODO add proper assign op operations like Operation but for now just do this
    };

    if !op_is_valid {
        return Err(
            ParseErrKind::IncompatiableOperation(
                Operation::Add,
                expr_type.get_name().unwrap(),
                rhs_type.get_name().unwrap()
            )
            .from_token(first_token)
        )
    }

    let expr_assign = ExprAssign::new(assign_op, Box::new(expr), Box::new(rhs));

    Ok(Expr::Assign(expr_assign))
}
