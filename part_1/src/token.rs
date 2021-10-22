use crate::token_kind::TokenKind;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub lexme: String,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, lexme: String, literal: Option<String>, line: usize) -> Token {
        Token {
            kind,
            lexme,
            literal,
            line,
        }
    }
}
