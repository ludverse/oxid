use crate::data::Data;
use crate::errors::ParseErrKind;
use crate::memory::Memory;
use crate::types::Type;

pub mod print;

const BUILTIN_FUNCTIONS: [(&'static str, Type, Data); 1] = [
    ("print", Type::BuiltinFn(BuiltinFn::Print), Data::BuiltinFn(BuiltinFn::Print)),
];

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinFn {
    Print,
}

impl BuiltinFn {
    pub fn populate_memory(memory: &mut Memory<Data>) {
        for (name, _, builtin_fn_data) in BUILTIN_FUNCTIONS {
            memory.insert(String::from(name), builtin_fn_data);
        }
    }

    pub fn populate_sim_memory(memory: &mut Memory<Type>) {
        for (name, builtin_fn_type, _) in BUILTIN_FUNCTIONS {
            memory.insert(String::from(name), builtin_fn_type);
        }
    }

    pub fn type_check(&self, args: Vec<Type>) -> Result<Type, ParseErrKind>{
        match self {
            BuiltinFn::Print => print::type_check(args)
        }
    }

    pub fn eval(&self, args: Vec<Data>) -> Data {
        match self {
            BuiltinFn::Print => print::eval(args)
        }
    }
}
