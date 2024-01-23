use std::collections::HashMap;

use crate::parser::Program;
use crate::data::Data;
use crate::statements::r#fn::FunctionDeclaration;

pub struct Interpreter {
    pub program: Program,
    pub memory: HashMap<String, Data>,
    pub functions: HashMap<String, FunctionDeclaration>
}

impl Interpreter {
    pub fn new(program: Program) -> Interpreter {
        Interpreter {
            program,
            memory: HashMap::new(),
            functions: HashMap::new()
        }
    }

    pub fn run(&mut self) {
        let statements = self.program.body.clone();
        for statement in statements.iter() {
            statement.exec(self);
        }
    }
}

