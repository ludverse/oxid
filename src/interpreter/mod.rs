use std::collections::HashMap;

use crate::statements::{Program, Statement};
use crate::expressions::data::Data;

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

    pub fn interpret_block(&mut self, statements: &Vec<Statement>) {
        let mut program_counter = 0;
        for i in 0..1_000_000 {
            match statements.get(program_counter) {
                Some(Statement::VariableAssignment(var_assignment)) => {
                    let val = var_assignment.init_value.eval(self);
                    self.memory.insert(var_assignment.name.to_string(), val);
                },
                Some(Statement::Expr(expr)) => {
                    expr.eval(self);
                },
                None => break
            }

            program_counter += 1;
        }
    }

    pub fn run(&mut self) {
        let statements = self.program.body.clone();
        self.interpret_block(&statements);
    }
}

