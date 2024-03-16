use crate::helpers::destructive_loop;

pub mod tokenizer;
pub mod parser;
pub mod interpreter;
pub mod memory;
pub mod statements;
pub mod expressions;
pub mod data;
pub mod operations;
pub mod types;
pub mod builtin;
pub mod errors;
pub mod helpers;

pub struct Config {
    pub source_file: String,
    pub std_file: Option<String>
}

impl Config {
    pub fn new(args: &mut impl Iterator<Item = String>) -> Config {
        args.next();

        let mut std_file = None;

        loop {
            match args.next() {
                Some(arg) => {

                    match &arg[..] {
                        "--std" => {
                            std_file = Some(args.next().expect("std file not specified"))
                        },
                        _ => {
                            return Config {
                                source_file: arg,
                                std_file
                            }
                        }
                    }

                },
                None => panic!("source file not specified")
            }
        }
    }
}

#[cfg(test)]
mod tests;
