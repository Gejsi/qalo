#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    // source: Source,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,

    Identifier,
    Int,
    String,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Equal,
    NotEqual,

    LessThan,
    GreaterThan,

    Comma,
    Semicolon,
    Colon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug)]
pub struct Source {
    filename: String,
    pos: (u32, u32),
}
