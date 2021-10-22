pub mod blocks;

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub index: usize,
    pub type_: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Function,
    Input,
    Symbol,
}

impl Token {
    pub fn new(value: String, end_index: usize, type_: TokenType) -> Self {
        Self {
            index: end_index - value.len(),
            value,
            type_,
        }
    }
}