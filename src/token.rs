use std::fmt;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub(crate) enum TokenType {
    // Single-character token.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    StringValue,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Identifier(s) => {
                if let Self::Identifier(o) = other {
                    s == o
                } else {
                    false
                }
            }
            Self::Str(s) => {
                if let Self::Str(o) = other {
                    s == o
                } else {
                    false
                }
            }
            Self::Number(s) => {
                if let Self::Number(o) = other {
                    s == o
                } else {
                    false
                }
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Literal {}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{:#?} {} {:#?}", self.r#type, self.lexeme, self.literal)
    }
}
