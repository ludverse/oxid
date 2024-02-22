use crate::builtin::BuiltinFn;
use crate::memory::Memory;
use crate::parser::Program;
use crate::data::Data;

pub struct Interpreter {
    pub program: Program,
    pub memory: Memory<Data>
}

impl Interpreter {
    pub fn new(program: Program) -> Interpreter {
        let mut memory = Memory::new();

        BuiltinFn::populate_memory(&mut memory);

        Interpreter {
            program,
            memory,
        }
    }

    pub fn run(&mut self) {
        let statements = self.program.body.clone();
        for statement in statements.iter() {
            statement.exec(self);
        }
    }
}

