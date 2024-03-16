use std::process::ExitCode;
use std::fs;
use std::env;
use oxid::Config;
use oxid::builtin::BuiltinFn;
use oxid::data::Data;
use oxid::interpreter::Interpreter;
use oxid::memory::Memory;
use oxid::parser::{Parser, TokenCollector};
use oxid::tokenizer::tokenize;
use oxid::types::Type;

fn execute_file(filename: &str, sim_memory: &mut Memory<Type>, memory: &mut Memory<Data>) {
    let buf = fs::read_to_string(filename)
        .expect(&format!("failed to read file `{}`", filename));
    let buf = buf.trim();

    let tokens = tokenize(&String::from("main"), buf);
    let collector = TokenCollector::new(&tokens);

    let mut parser = Parser::new(collector, sim_memory);
    let statements = parser.generate_program();
    let mut interpreter = Interpreter::new(&statements, memory);
    interpreter.run_program();
}

fn main() -> ExitCode {
    let mut args = env::args();
    let config = Config::new(&mut args);

    let mut sim_memory = Memory::new();
    BuiltinFn::populate_sim_memory(&mut sim_memory);

    let mut memory = Memory::new();
    BuiltinFn::populate_memory(&mut memory);

    let std_file = config.std_file
        .unwrap_or(String::from("std.ox"));
    execute_file(&std_file[..], &mut sim_memory, &mut memory);
    execute_file(&config.source_file[..], &mut sim_memory, &mut memory);

    0.into()
}
