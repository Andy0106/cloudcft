use super::definetion::TokenType;
pub struct Token(pub TokenType, pub usize, pub Option<String>);

#[allow(unused)]
pub struct Lexer {
    src: String,
    line: usize,
    ptr: usize,
    pub next_token_info: TokenType,
    next_token: Token,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            src: source,
            line: 0,
            ptr: 0,
            next_token_info: TokenType::None,
            next_token: Token(TokenType::None, 0, None),
        }
    }
    pub fn scan_name(lexer: &Lexer) -> (usize, String) {
        let codes = &lexer.src[lexer.ptr..];
        let mut i = 0;
        for c in codes.chars() {
            match c {
                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => i += 1,
                _ => break,
            }
        }
        (i, codes[..i].to_string())
    }
}
