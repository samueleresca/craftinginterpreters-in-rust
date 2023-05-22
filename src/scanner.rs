use std::any::Any;

use crate::{
    errors::{CompileError, WithError},
    token::{
        self, Token,
        TokenType::{self},
    },
};

struct Scanner {
    tokens: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl WithError for Scanner {}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !&self.is_at_end() {
            self.start = self.current;
            let result = self.scan_token();

            if let Err(e) = result {
                // We don't want to interrupt the scan.
                Self::error(e);
            }
        }

        let token = Token {
            r#type: token::TokenType::Eof,
            lexeme: String::from(""),
            literal: None,
            line: self.line,
        };

        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        &self.current >= &(self.source.len())
    }

    fn scan_token(&mut self) -> Result<(), CompileError> {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = if self.match_token('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.match_token('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.match_token('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.match_token('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            }
            '/' => {
                if self.match_token('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            _ => {
                return Err(CompileError::new(
                    self.line as usize,
                    "".to_string(),
                    "Unexpected character.".to_string(),
                ))
            }
        };
        Ok(())
    }

    fn add_token(&mut self, r#type: token::TokenType) {
        self.add_token_literal(r#type, None);
    }

    fn add_token_literal(&mut self, r#type: token::TokenType, literal: Option<Box<dyn Any>>) {
        let text = &self.source[self.start as usize..self.current as usize];
        let token = Token {
            r#type,
            lexeme: String::from(text),
            literal,
            line: self.line,
        };
        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize);
        self.current += 1;
        c.unwrap()
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize) != Some(expected) {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        self.source.chars().nth(self.current as usize)
    }
}
