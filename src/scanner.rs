use std::{ops::IndexMut, thread::current};

use crate::token::{self, Token, TokenType};

struct Scanner {
    tokens: Vec<Token>,
    source: String,
    start: i32,
    current: i32,
    line: i32,
}

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

    pub fn scan_tokens(&self) -> Vec<Token> {
        while !&self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let token = Token {
            r#type: token::TokenType.Eof,
            lexeme: "",
            literal: Nil,
            line: todo!(),
        };
        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        &self.current >= &(self.source.len() as i32)
    }

    fn scan_token(&self) {}
}
