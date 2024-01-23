use crate::parser::Parser;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::Expr;
use crate::tokenizer::{Token, TokenType};

pub use path::*;
pub use method::*;
pub use assign::*;

mod path;
mod method;
mod assign;

pub fn parse(parser: &mut Parser, first_token: &Token) -> Result<Expr, ParseErr> {
    match &first_token.token {
        TokenType::Identifier(name) => match &parser.collector.next().token {

            TokenType::LeftParen => method::parse(parser, first_token, name),
            assign_token if assign_token.to_assign_op().is_some() => {

                let operation = assign_token.to_assign_op().unwrap();

                let next_token = parser.collector.next();
                let expr = Expr::parse_expr(parser, next_token)?;

                if !parser.sim_memory.contains_key(name) {
                    return Err(ParseErrKind::UnknownField(name.to_string()).to_err(first_token.pos));
                }
                Ok(Expr::Assign(ExprAssign::new(operation, vec![name.to_string()], Box::new(expr))))

            },
            _ => {

                parser.collector.back();

                if !parser.sim_memory.contains_key(name) {
                    return Err(ParseErrKind::UnknownField(name.to_string()).to_err(first_token.pos));
                }
                Ok(Expr::Path(ExprPath::new(vec![name.to_string()])))

            }
        },
        _ => panic!("this method should not have been able to be called if it's not an Identifier")
    }
}
