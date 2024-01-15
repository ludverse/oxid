pub mod tokenizer;
pub mod parser;
// pub mod memory;
pub mod interpreter;
pub mod statements;
pub mod expressions;
pub mod errors;

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

pub fn report(err_code: i32, err_msg: String) -> ! {
    panic!("[{}] error: {}", err_code, err_msg)
}

pub fn report_pos(err_code: i32, err_msg: String, char_pos: usize) -> ! {
    panic!("[{}] error (char: {}): {}", err_code, char_pos, err_msg)
}

