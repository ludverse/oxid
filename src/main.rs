use std::process::ExitCode;
use std::fs;
use std::env;
use oxid::Config;
use oxid::interpreter::Interpreter;
use oxid::parser::TokenCollector;
use oxid::tokenizer::tokenize;
use oxid::parser::Parser;
use oxid::statements::Statement;

fn main() -> ExitCode {
    let mut args = env::args();
    let config = Config::new(&mut args);

    let buf = fs::read_to_string(config.source_file)
        .expect("failed to read source file");
    let buf = buf.trim();

    let tokens = tokenize(buf);
    let collector = TokenCollector::new(&tokens);

    let mut parser = Parser::new(collector);
    let program = parser.generate_program();

    // dbg!(&program);

    let mut interpreter = Interpreter::new(program);
    interpreter.run();

    0.into()
}
