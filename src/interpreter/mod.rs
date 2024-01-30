use crate::memory::Memory;
use crate::parser::Program;
use crate::data::Data;

pub struct Interpreter {
    pub program: Program,
    pub memory: Memory<Data>
}

impl Interpreter {
    pub fn new(program: Program) -> Interpreter {
        Interpreter {
            program,
            memory: Memory::new()
        }
    }

    pub fn run(&mut self) {
        let statements = self.program.body.clone();
        for statement in statements.iter() {
            statement.exec(self);
        }
    }
}

