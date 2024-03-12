use crate::memory::Memory;
use crate::data::Data;
use crate::statements::Statement;

pub struct Interpreter<'a, 'm> {
    pub statements: &'a Vec<Statement>,
    pub memory: &'m mut Memory<Data>
}

impl<'a, 'm> Interpreter<'a, 'm> {
    pub fn new(statements: &'a Vec<Statement>, memory: &'m mut Memory<Data>) -> Self {
        Self {
            statements,
            memory,
        }
    }

    pub fn run_program(&mut self) {
        for statement in self.statements.into_iter() {
            statement.exec(self);
        }
    }
}

