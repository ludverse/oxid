use crate::errors::ParseErr;
use crate::tokenizer::Token;
use crate::parser::{
    match_tree,
    Parser
};
use crate::expressions::Expr;
use crate::statements::Statement;

#[derive(Debug, Clone)]
pub struct VariableAssignment {
    pub name: String,
    pub init_value: Box<Expr>,
    pub is_mut: bool
}

impl VariableAssignment {
    fn new(name: String, init_value: Box<Expr>, is_mut: bool) -> VariableAssignment {
        VariableAssignment {
            name,
            init_value,
            is_mut
        }
    }
}

pub fn parse(interpreter: &mut Parser) -> Result<Statement, ParseErr> {
    let mut next_token = interpreter.collector.next();
    let mut is_mut = false;

    if let Token::Mut = next_token {
        is_mut = true;
        next_token = interpreter.collector.next();
    }

    match_tree!(next_token, Token::Identifier(name), interpreter, "variable name", 
        match_tree!(interpreter.collector.next(), Token::Equal, interpreter, "Equal", {
            let expr_token = interpreter.collector.next();
            let expr = interpreter.parse_expr(expr_token)?;

            interpreter.sim_memory.insert(name.to_string(), expr.clone());

            Ok(Statement::VariableAssignment(VariableAssignment::new(name.to_string(), expr, is_mut)))
        })
    )
}
