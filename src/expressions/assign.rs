use crate::data::Data;
use crate::errors::ParseErrKind;
use crate::expressions::path::Field;
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
    field: Field,
    value: Box<Expr>
}

impl ExprAssign {
    pub fn new(op: AssignOp, field: Field, value: Box<Expr>) -> ExprAssign {
        ExprAssign {
            op,
            field,
            value
        }
    }
}

impl Evaluable for ExprAssign {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let mangled = self.field.mangle();

        let value = self.value.typ(parser)?;
        let old = parser.sim_memory.get(&mangled)
            .ok_or(ParseErrKind::UnknownField(mangled))?;

        match self.op {
            AssignOp::Eq => self.value.typ(parser),
            AssignOp::AddEq => Operation::Add.typ(old, &value)
        }
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let mangled = self.field.mangle();

        let value = self.value.eval(interpreter);
        let old = interpreter.memory.get(&mangled).unwrap();

        let data = match self.op {
            AssignOp::Eq => value,
            AssignOp::AddEq => Operation::Add.op(old, &value)
        };

        interpreter.memory.assign(mangled, data.clone());

        data
    }
}
