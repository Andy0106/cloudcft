use std::collections::HashMap;
pub enum token_type {
    EOF,
    NAME,
    DATA_TYPE,
    IGNORE,
    VARIABLE,
    STM_END,
    EQUAL,
}
pub struct Token(pub token_type, pub usize, pub Option<String>);

pub struct Lexer {
    src: Vec<String>,
    line: usize,
    ptr: usize,
    pub next_token_info: token_type,
    next_token: Token,
    words: HashMap<String, Token>,
}
impl Lexer {
    pub fn new(source: String) {}
}
