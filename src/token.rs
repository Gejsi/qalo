#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    // TODO: add source location for more accurate debugging info
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
    LessThanEqual,
    GreaterThanEqual,

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

impl TokenKind {
    /// Matches keywords.
    pub fn lookup_identifier(identifier: &str) -> TokenKind {
        match identifier {
            "fn" => TokenKind::Function,
            "let" => TokenKind::Let,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Source {
    filename: String,
    pos: (u32, u32),
}
