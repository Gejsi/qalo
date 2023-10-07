#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    // TODO: use `Cow`
    pub literal: String,
    // TODO: add source location for more accurate debugging info
    // source: Source,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Illegal,
    Eof,

    Identifier,
    Integer,
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
    LeftSquare,
    RightSquare,

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

// #[derive(Debug, PartialEq, Eq)]
// pub struct Source {
//     filename: String,
//     pos: (u32, u32),
// }
