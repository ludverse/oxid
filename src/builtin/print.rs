use crate::data::Data;
use crate::errors::ParseErrKind;

use crate::types::Type;

pub fn type_check(_args: Vec<Type>) -> Type {
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
