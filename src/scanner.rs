use crate::{
    errors::{CompileError, WithError},
    token::{
        self, Literal, Token,
        TokenType::{self},
    },
};

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Scanner {
    tokens: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
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
            keywords: vec![
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),
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
            r#type: TokenType::Eof,
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
        let c = self.advance();
        match c {
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
            ' ' | '\r' | '\t' => {
                // Ignore.
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                let result = self.parse_string();
                if result.is_err() {
                    return result;
                }
            }
            _ => {
                if Scanner::is_digit(Some(c)) {
                    let result = self.parse_number();
                    if result.is_err() {
                        return result;
                    }
                    return Ok(());
                }

                if Scanner::is_alpha(Some(c)) {
                    let result = self.parse_identifier();

                    if result.is_err() {
                        return result;
                    }
                    return Ok(());
                }

                return Err(CompileError::new(
                    self.line as usize,
                    "".to_string(),
                    "Unexpected character.".to_string(),
                ));
            }
        };
        Ok(())
    }

    fn add_token(&mut self, r#type: token::TokenType) {
        self.add_token_literal(r#type, None);
    }

    fn add_token_literal(&mut self, r#type: token::TokenType, literal: Option<Literal>) {
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

    fn parse_string(&mut self) -> Result<(), CompileError> {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(CompileError::new(
                self.line,
                "".to_string(),
                "Unterminated string.".to_string(),
            ));
        }

        self.advance();
        let value = self
            .source
            .get(self.start + 1..self.current - 1)
            .map(|f| f.to_string());

        let literal_value: Option<Literal> = value.map(|v| Literal::Str(v));
        self.add_token_literal(TokenType::StringValue, literal_value);

        Ok(())
    }

    fn parse_number(&mut self) -> Result<(), CompileError> {
        while Scanner::is_digit(self.peek()) {
            self.advance();
            return Ok(());
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self
            .source
            .get(self.start..self.current)
            .map(|f| f.to_string());

        let value_parsed = value
            .map(|v| v.parse::<f64>().unwrap())
            .map(|v| Literal::Number(v));
        self.add_token_literal(TokenType::Number, value_parsed);

        Ok(())
    }

    fn parse_identifier(&mut self) -> Result<(), CompileError> {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.source.get(self.start..self.current);

        if text.is_none() {
            return Err(CompileError::new(
                self.line,
                "".to_string(),
                "Unexpected character.".to_string(),
            ));
        }

        let token_type = match self.keywords.get(text.unwrap()) {
            Some(kw_token_type) => *kw_token_type,
            None => TokenType::Identifier,
        };

        match token_type {
            TokenType::Identifier => self.add_token_literal(
                TokenType::Identifier,
                Some(Literal::Identifier(text.unwrap().to_string())),
            ), // book doesn't do this. why not?}
            _ => self.add_token(token_type),
        }

        Ok(())
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

    fn is_digit(c: Option<char>) -> bool {
        match c {
            Some(c) => c >= '0' && c <= '9',
            _ => false,
        }
    }

    fn is_alpha(c: Option<char>) -> bool {
        match c {
            Some(c) => (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_',
            _ => false,
        }
    }

    fn is_alphanumeric(c: Option<char>) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        self.source.chars().nth(self.current as usize)
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return Some('\0');
        }
        self.source.chars().nth(self.current + 1 as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        errors::{WithError},
        token::{Literal, TokenType},
    };

    use super::Scanner;

    #[test]
    fn parse_empty() {
        let mut scanner = Scanner::new("".to_owned());
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].r#type, TokenType::Eof);
    }

    #[test]
    fn parse_error() {
        let mut scanner = Scanner::new(r#"vackfdsf; ""#.to_owned());
        scanner.scan_tokens();

        assert_eq!(Scanner::has_error(), true);
        assert_eq!(scanner.tokens.len(), 3);
        assert_eq!(scanner.tokens[0].r#type, TokenType::Identifier);
        assert_eq!(scanner.tokens[1].r#type, TokenType::Semicolon);
        assert_eq!(scanner.tokens[2].r#type, TokenType::Eof);
    }

    #[test]
    fn parse_addition() {
        let mut scanner = Scanner::new("1 + 2".to_owned());
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 4);
        assert_eq!(scanner.tokens[0].r#type, TokenType::Number);
        assert_eq!(scanner.tokens[1].r#type, TokenType::Plus);
        assert_eq!(scanner.tokens[2].r#type, TokenType::Number);
        assert_eq!(scanner.tokens[3].r#type, TokenType::Eof);
    }

    #[test]
    fn parse_subtractions() {
        let mut scanner = Scanner::new("1 - 2".to_owned());
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 4);
        assert_eq!(scanner.tokens[0].r#type, TokenType::Number);
        assert_eq!(
            scanner.tokens[0].literal.as_ref().unwrap(),
            &Literal::Number(1.0)
        );
        assert_eq!(scanner.tokens[1].r#type, TokenType::Minus);
        assert_eq!(scanner.tokens[2].r#type, TokenType::Number);
        assert_eq!(
            scanner.tokens[2].literal.as_ref().unwrap(),
            &Literal::Number(2.0)
        );
        assert_eq!(scanner.tokens[3].r#type, TokenType::Eof);
    }

    #[test]
    fn parse_decimal() {
        let mut scanner = Scanner::new("1.3 - 2.2".to_owned());
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 4);
        assert_eq!(scanner.tokens[0].r#type, TokenType::Number);
        assert_eq!(
            scanner.tokens[0].literal.as_ref().unwrap(),
            &Literal::Number(1.3)
        );
        assert_eq!(scanner.tokens[1].r#type, TokenType::Minus);
        assert_eq!(scanner.tokens[2].r#type, TokenType::Number);
        assert_eq!(
            scanner.tokens[2].literal.as_ref().unwrap(),
            &Literal::Number(2.2)
        );
        assert_eq!(scanner.tokens[3].r#type, TokenType::Eof);
    }

    #[test]
    fn parse_string() {
        let mut scanner = Scanner::new(r#""test""#.to_owned());
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].r#type, TokenType::StringValue);
        assert_eq!(
            scanner.tokens[0].literal.as_ref().unwrap(),
            &Literal::Str("test".to_string())
        );
        assert_eq!(scanner.tokens[1].r#type, TokenType::Eof);
    }

    #[test]
    fn parse_variable_defintion() {
        let mut scanner = Scanner::new(r#"var mytest = "test""#.to_owned());
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].r#type, TokenType::Var);
        assert_eq!(scanner.tokens[1].r#type, TokenType::Identifier);
        assert_eq!(
            scanner.tokens[1].literal.as_ref().unwrap(),
            &Literal::Identifier("mytest".to_string())
        );
        assert_eq!(scanner.tokens[2].r#type, TokenType::Equal);
        assert_eq!(scanner.tokens[3].r#type, TokenType::StringValue);
        assert_eq!(
            scanner.tokens[3].literal.as_ref().unwrap(),
            &Literal::Str("test".to_string())
        );
        assert_eq!(scanner.tokens[4].r#type, TokenType::Eof);
    }
}
