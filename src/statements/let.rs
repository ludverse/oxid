use crate::errors::ParseErr;
use crate::interpreter::Interpreter;
use crate::tokenizer::{Token, TokenType};
use crate::parser::Parser;
use crate::expressions::Expr;
use crate::statements::{Executable, ParseableStatement, Statement};

#[derive(Debug, Clone)]
pub struct VariableAssignment {
    pub name: String,
    pub init_value: Expr,
    pub is_mut: bool
}

impl VariableAssignment {
    fn new(name: String, init_value: Expr, is_mut: bool) -> VariableAssignment {
        VariableAssignment {
            name,
            init_value,
            is_mut
        }
    }
}

impl Executable for VariableAssignment {
    fn exec(&self, interpreter: &mut Interpreter) {
        let val = self.init_value.eval(interpreter);
        interpreter.memory.insert(self.name.to_string(), val);
    }
}

impl ParseableStatement for VariableAssignment {
    fn parse(parser: &mut Parser, _first_token: &Token) -> Result<Statement, ParseErr> {
        let mut next_token = parser.collector.next();
        let mut is_mut = false;

        if let TokenType::Mut = next_token.token {
            is_mut = true;
            next_token = parser.collector.next();
        }

        match &next_token.token {
            TokenType::Identifier(name) => {

                let next_token = parser.collector.next();
                match next_token.token {
                    TokenType::Equal => {

                        let expr_token = parser.collector.next();
                        let expr = Expr::parse_expr(parser, expr_token)?;
                        let expr_type = expr.get_type(parser)
                            .map_err(|err_kind| err_kind.to_err(expr_token.pos))?;

                        parser.sim_memory.insert(name.to_string(), expr_type);

                        Ok(Statement::VariableAssignment(VariableAssignment::new(name.to_string(), expr, is_mut)))

                    },
                    _ => Err(parser.unexpected_token(next_token, "Equal"))
                }

            },
            _ => Err(parser.unexpected_token(next_token, "variable name"))
        }
    }
}
