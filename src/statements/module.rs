use std::fs;

use crate::errors::ParseErr;
use crate::interpreter::Interpreter;
use crate::tokenizer::tokenize;
use crate::tokenizer::{token::Token, token_type::TokenType};
use crate::parser::{Parser, TokenCollector};
use crate::statements::{Executable, ParseableStatement, Statement};

#[derive(Debug, Clone)]
pub struct ModuleImport {
    pub name: String,
    pub statements: Vec<Statement>
}

impl ModuleImport {
    fn new(name: String, statements: Vec<Statement>) -> ModuleImport {
        ModuleImport {
            name,
            statements
        }
    }
}

impl Executable for ModuleImport {
    fn exec(&self, interpreter: &mut Interpreter) {
        let mut mod_interpreter = Interpreter::new(&self.statements, interpreter.memory);
        mod_interpreter.run_program();
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

                let mut buf_parser = Parser::new(collector, parser.sim_memory);

                let statements = buf_parser.generate_program();

                // TODO do error handling stuff for this
                Ok(Statement::ModuleImport(ModuleImport::new(name.to_string(), statements)))
            },
            _ => Err(parser.unexpected_token(next_token, "variable name"))
        }
    }
}
