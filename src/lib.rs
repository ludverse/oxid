pub mod tokenizer;
pub mod parser;
// pub mod memory;
pub mod interpreter;
pub mod statements;
pub mod expressions;
pub mod data;
pub mod operations;
pub mod types;
pub mod builtin;
pub mod errors;
pub mod helpers;

pub struct Config {
    pub source_file: String
}

impl Config {
    pub fn new(args: &mut impl Iterator<Item = String>) -> Config {
        args.next();

        let source_file = args.next()
            .expect("source_file not specified");

        Config {
            source_file
        }
    }
}

#[cfg(test)]
mod tests;
