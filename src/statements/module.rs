use std::fs;
use std::mem;

use crate::errors::{map_err_token, ParseErr};
use crate::interpreter::Interpreter;
use crate::tokenizer::tokenize;
use crate::tokenizer::{token::Token, token_type::TokenType};
use crate::parser::{Parser, TokenCollector, Program};
use crate::expressions::Expr;
use crate::statements::{Executable, ParseableStatement, Statement};

#[derive(Debug, Clone)]
pub struct ModuleImport {
    pub name: String,
    pub program: Program
}

impl ModuleImport {
    fn new(name: String, program: Program) -> ModuleImport {
        ModuleImport {
            name,
            program
        }
    }
}

impl Executable for ModuleImport {
    fn exec(&self, interpreter: &mut Interpreter) {
        let mut interpreter = Interpreter::new(self.program.clone());
        interpreter.run();
    }
}

impl ParseableStatement for ModuleImport {
    fn parse(parser: &mut Parser, _first_token: &Token) -> Result<Statement, ParseErr> {
        let next_token = parser.collector.next();
        match &next_token.token {
            TokenType::Identifier(name) => {

                let mut filename = name.to_string();
                filename.push_str(".ox");

                let buf = fs::read_to_string(filename)
                    .expect("failed to read source file");
                let buf = buf.trim();

                let tokens = tokenize(name, buf);
                let collector = TokenCollector::new(&tokens);

                let old_collector = mem::replace(&mut parser.collector, collector);
                let program = parser.generate_program();
                parser.collector = old_collector;

                // TODO do error handling stuff for this
                Ok(Statement::ModuleImport(ModuleImport::new(name.to_string(), program)))
            },
            _ => Err(parser.unexpected_token(next_token, "variable name"))
        }
    }
}
