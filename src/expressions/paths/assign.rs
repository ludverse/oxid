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
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let value = self.value.get_type(parser)?;
        let old = parser.sim_memory.get(&self.path[0])
            .ok_or(ParseErrKind::UnknownField(self.path[0].clone()))?;

        match self.op {
            AssignOp::Eq => self.value.get_type(parser),
            AssignOp::AddEq => Operation::Add.typ(old, &value)
        }
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let value = self.value.eval(interpreter);
        let old = interpreter.memory.get(&self.path[0]).unwrap();

        let data = match self.op {
            AssignOp::Eq => value,
            AssignOp::AddEq => Operation::Add.op(old, &value)
        };

        interpreter.memory.assign(self.path[0].to_string(), data.clone());

        data
    }
}
