use std::collections::HashMap;
use super::definetion::token_type;
pub struct Token(pub token_type, pub usize, pub Option<String>);

pub struct Lexer {
    src: String,
    line: usize,
    ptr: usize,
    pub next_token_info: token_type,
    next_token: Token
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            src: source,
            line: 0,
            ptr: 0,
            next_token_info: token_type::NONE,
            next_token: Token(token_type::NONE,0,None)
        }
    }
}

