use crate::data::Data;
use crate::errors::ParseErrKind;
use crate::expressions::{Expr, Evaluable};
use crate::interpreter::Interpreter;
use crate::operations::Operation;
use crate::parser::Parser;
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
        let mangled = self.lhs.mangle_path()?;

        let value = self.rhs.typ(parser)?;
        let old = parser.sim_memory.get(&mangled)
            .ok_or(ParseErrKind::UnknownField(mangled))?;

        match self.op {
            AssignOp::Eq => self.rhs.typ(parser),
            AssignOp::AddEq => Operation::Add.typ(old, &value)
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
