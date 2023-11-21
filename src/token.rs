use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
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
    Percentage,

    // TODO: add && and ||
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

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "illegal"),
            TokenKind::Eof => write!(f, "eof"),

            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::Integer => write!(f, "integer"),
            TokenKind::String => write!(f, "string"),

            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percentage => write!(f, "%"),

            TokenKind::Equal => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::LessThanEqual => write!(f, "<="),
            TokenKind::GreaterThanEqual => write!(f, ">="),

            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftSquare => write!(f, "["),
            TokenKind::RightSquare => write!(f, "]"),

            TokenKind::Function => write!(f, "fn"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Return => write!(f, "return"),
        }
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct Source {
//     filename: String,
//     pos: (u32, u32),
// }
