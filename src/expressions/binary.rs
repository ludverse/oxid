use crate::operations::Operation;
use crate::parser::Parser;
use crate::types::Type;
use crate::errors::{ParseErrKind, ParseErr, map_err_token};
use crate::data::Data;
use crate::interpreter::Interpreter;
use crate::tokenizer::token::Token;

use super::{Expr, Evaluable};

#[derive(Debug, Clone)]
pub struct ExprBinary {
    operation: Operation,
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl ExprBinary {
    pub fn new(operation: Operation, lhs: Box<Expr>, rhs: Box<Expr>) -> ExprBinary {
        ExprBinary {
            operation,
            lhs,
            rhs
        }
    }
}

impl Evaluable for ExprBinary {
    fn type_check(&self, parser: &Parser) -> Type {
        let lhs = self.lhs.type_check(parser);
        let rhs = self.rhs.type_check(parser);

        self.operation.typ(&lhs, &rhs).unwrap()
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let lhs = self.lhs.eval(interpreter);
        let rhs = self.rhs.eval(interpreter);

        self.operation.op(&lhs, &rhs)
    }
}

pub fn parse(parser: &mut Parser, first_token: &Token, expr: Expr, operation: Operation) -> Result<Expr, ParseErr> {
    let expr_type = expr.type_check(parser);

    let rhs_token = parser.collector.next();
    let rhs = Expr::parse_expr_side(parser, rhs_token)?;
    let rhs_type = rhs.type_check(parser);

    operation.typ(&expr_type, &rhs_type)
        .ok_or_else(|| 
            ParseErrKind::IncompatiableOperation(operation, expr_type.get_name().unwrap(), rhs_type.get_name().unwrap())
                .from_token(first_token)
        )?;

    let binary_expr = ExprBinary::new(operation, Box::new(expr), Box::new(rhs));
    Ok(Expr::Binary(binary_expr))
}
