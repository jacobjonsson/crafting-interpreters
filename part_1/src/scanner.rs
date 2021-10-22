use crate::token::Token;
use crate::token_kind::TokenKind;

#[derive(Debug)]
pub enum ScannerErrorKind {
    UnexpectedToken,
}

#[derive(Debug)]
pub struct ScannerError {
    pub kind: ScannerErrorKind,
    pub line: usize,
}

impl ScannerError {
    pub fn new(kind: ScannerErrorKind, line: usize) -> ScannerError {
        ScannerError { kind, line }
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_token(&mut self) -> Result<Token, ScannerError> {
        self.start = self.current;

        if self.is_at_end() {
            return Ok(self.create_token(TokenKind::EOF, None));
        }

        let token = match self.advance() {
            '(' => self.create_token(TokenKind::LeftParen, None),
            ')' => self.create_token(TokenKind::RightParen, None),
            '{' => self.create_token(TokenKind::LeftBrace, None),
            '}' => self.create_token(TokenKind::RightBrace, None),
            ',' => self.create_token(TokenKind::Comma, None),
            '.' => self.create_token(TokenKind::Dot, None),
            '-' => self.create_token(TokenKind::Minus, None),
            '+' => self.create_token(TokenKind::Plus, None),
            ';' => self.create_token(TokenKind::Semicolon, None),
            '*' => self.create_token(TokenKind::Star, None),

            '!' => {
                if self.r#match('=') {
                    self.create_token(TokenKind::BangEqual, None)
                } else {
                    self.create_token(TokenKind::Bang, None)
                }
            }

            '=' => {
                if self.r#match('=') {
                    self.create_token(TokenKind::EqualEqual, None)
                } else {
                    self.create_token(TokenKind::Equal, None)
                }
            }

            '<' => {
                if self.r#match('=') {
                    self.create_token(TokenKind::LessEqual, None)
                } else {
                    self.create_token(TokenKind::Less, None)
                }
            }

            '>' => {
                if self.r#match('=') {
                    self.create_token(TokenKind::GreaterEqual, None)
                } else {
                    self.create_token(TokenKind::Greater, None)
                }
            }

            '/' => {
                if self.r#match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return self.scan_token();
                } else {
                    self.create_token(TokenKind::Slash, None)
                }
            }

            '\n' => {
                self.line += 1;
                return self.scan_token();
            }

            ' ' | '\r' | '\t' => {
                return self.scan_token();
            }

            _ => {
                return Err(ScannerError::new(
                    ScannerErrorKind::UnexpectedToken,
                    self.line,
                ))
            }
        };

        Ok(token)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1..]
            .chars()
            .next()
            .unwrap_or('\0')
    }

    fn r#match(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != ch {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        self.source[self.current..].chars().next().unwrap_or('\0')
    }

    fn create_token(&mut self, kind: TokenKind, literal: Option<String>) -> Token {
        let text = &self.source[self.start..self.current];
        Token::new(kind, text.into(), literal, self.line)
    }
}
