use crate::data::Data;
use crate::interpreter::Interpreter;
use crate::types::Type;

pub fn arg_types() -> &'static [(&'static str, Type)] {
    &[("string", Type::String)]
}

pub fn return_type() -> Type {
    Type::TempNil
}

pub fn eval(args: Vec<Data>) -> Data {
    let arg = args.first().unwrap();
    
    match arg {
        Data::String(arg) => println!("{}", arg),
        Data::Number(arg) => println!("{:?}", arg),
        _ => panic!("should have already been caught by the parser")
    }

    Data::TempNil
}
