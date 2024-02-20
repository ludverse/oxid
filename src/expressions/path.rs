use crate::helpers::destructive_loop;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::errors::{ParseErrKind, ParseErr};
use crate::expressions::{Expr, Evaluable};
use crate::data::Data;
use crate::operations::Operation;
use crate::tokenizer::{TokenType, Token};
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Field {
    field_name: String,
    parent: Option<Box<Field>>
}

impl Field {
    pub fn new(field_name: String, parent: Option<Box<Field>>) -> Self {
        Self {
            field_name,
            parent
        }
    }

    pub fn mangle(&self) -> String {
        let mut res = String::new();

        if let Some(parent) = &self.parent {
            res.push_str(&parent.mangle()[..]);
            res.push('.');
        }

        res.push_str(&self.field_name[..]);

        res
    }
}

#[derive(Debug, Clone)]
pub struct ExprPath {
    pub field: Field
}

impl ExprPath {
    pub fn new(field: Field) -> Self {
        Self {
            field
        }
    }
}

impl Evaluable for ExprPath {
    fn typ(&self, parser: &Parser) -> Result<Type, ParseErrKind> {
        let mangled = self.field.mangle();
        dbg!(&parser.sim_memory.scopes);
        parser.sim_memory.get(&mangled)
            .ok_or(ParseErrKind::UnknownField(mangled))
            .cloned()
    }

    fn eval(&self, interpreter: &mut Interpreter) -> Data {
        let mangled = self.field.mangle();
        interpreter.memory.get(&mangled).unwrap().clone()
    }
}

pub fn parse(parser: &mut Parser, first_token: &Token) -> Result<Expr, ParseErr> {
    let mut next_token = first_token;

    let mut field = None;

    destructive_loop!({

        match &next_token.token {
            TokenType::Identifier(field_name) => {

                let parent = field.map(|parent| Box::new(parent));
                field = Some(Field::new(field_name.to_string(), parent));

                let dot_token = parser.collector.next();
                match &dot_token.token {
                    TokenType::Dot => next_token = parser.collector.next(),
                    _ => {
                        parser.collector.back();
                        break
                    }
                }

            },
            _ => return Err(parser.unexpected_token(next_token, "field"))
        }

    });

    Ok(Expr::Path(ExprPath::new(field.unwrap())))
}
