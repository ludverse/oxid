use crate::tokenizer::Token;
#[derive(Debug, Clone)]
pub struct DataBlock {
    pub name: String,
    pub data_type: Data,
    pub is_mut: bool
}

impl DataBlock {
    pub fn new(name: String, data_type: Data, is_mut: bool) -> DataBlock {
        DataBlock {
            name,
            data_type,
            is_mut
        }
    }
}
