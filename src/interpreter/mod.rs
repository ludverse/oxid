use std::collections::HashMap;

use crate::parser::Program;
use crate::data::Data;

pub struct Interpreter {
    pub program: Program,
    pub memory: HashMap<String, Data>
}

impl Interpreter {
    pub fn new(program: Program) -> Interpreter {
        let memory = HashMap::new();
        Interpreter {
            program,
            memory
        }
    }

    pub fn run(&mut self) {
        let statements = self.program.body.clone();
        for statement in statements.iter() {
            statement.exec(self);
        }
    }
}

