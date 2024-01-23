use crate::parser::Parser;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::Expr;
use crate::tokenizer::Token;

pub use path::*;
pub use method::*;
pub use assign::*;

mod path;
mod method;
mod assign;

pub fn parse(parser: &mut Parser, name: &String) -> Result<Expr, ParseErr> {
    let name_pos = parser.collector.current_pos();

    match parser.collector.next() {
        Token::LeftParen => method::parse(parser, name, name_pos),
        token if token.to_assign_op().is_some() => {
            let operation = token.to_assign_op().unwrap();

            let next_token = parser.collector.next();
            let expr = Expr::parse_expr(parser, next_token)?;

            if !parser.sim_memory.contains_key(name) {
                return Err(ParseErrKind::UnknownField(name.to_string()).to_err(name_pos));
            }
            Ok(Expr::Assign(ExprAssign::new(operation, vec![name.to_string()], Box::new(expr))))
        },
        _ => {
            parser.collector.back();

            if !parser.sim_memory.contains_key(name) {
                return Err(ParseErrKind::UnknownField(name.to_string()).to_err(name_pos));
            }
            Ok(Expr::Path(ExprPath::new(vec![name.to_string()])))
        }
    }
}
