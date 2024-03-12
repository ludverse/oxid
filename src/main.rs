use std::process::ExitCode;
use std::fs;
use std::env;
use oxid::Config;
use oxid::builtin::BuiltinFn;
use oxid::interpreter::Interpreter;
use oxid::memory::Memory;
use oxid::parser::{Parser, TokenCollector};
use oxid::tokenizer::tokenize;


fn main() -> ExitCode {
    let mut args = env::args();
    let config = Config::new(&mut args);

    let main_buf = fs::read_to_string(config.source_file)
        .expect("failed to read source file");
    let buf = main_buf.trim();

    let tokens = tokenize(&String::from("main"), buf);
    let collector = TokenCollector::new(&tokens);

    let mut sim_memory = Memory::new();
    BuiltinFn::populate_sim_memory(&mut sim_memory);

    let mut main_parser = Parser::new(collector, &mut sim_memory);
    let statements = main_parser.generate_program();

    let mut memory = Memory::new();
    BuiltinFn::populate_memory(&mut memory);

    let mut main_interpreter = Interpreter::new(&statements, &mut memory);
    main_interpreter.run_program();

    0.into()
}
