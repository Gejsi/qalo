use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    curr: usize, // current position in input (points to current char)
    next: usize, // next position in input (after current char)
    ch: char,    // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            curr: 0,
            next: 0,
            ch: '\0',
        };

        lexer.read_char();
        lexer
    }

    /// Give the next character and advance position in the input string.
    pub fn read_char(&mut self) {
        self.ch = if self.next >= self.input.chars().count() {
            // reached EOF
            '\0'
        } else {
            self.input.chars().nth(self.next).unwrap_or('\0')
        };

        self.curr = self.next;
        self.next += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' => Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            '+' => Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            '(' => Token {
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
            },
            ')' => Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
            },
            '{' => Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            '}' => Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
            ';' => Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ',' => Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            '\0' => Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
            _ => Token {
                kind: TokenKind::Illegal,
                literal: "".to_string(),
            },
        };

        self.read_char();

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn next_token() {
        let input = "=+(){};,";

        let tests = vec![
            (TokenKind::Assign, "="),
            (TokenKind::Plus, "+"),
            (TokenKind::LeftParen, "("),
            (TokenKind::RightParen, ")"),
            (TokenKind::LeftBrace, "{"),
            (TokenKind::RightBrace, "}"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Comma, ","),
            (TokenKind::Eof, ""),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (i, (expected_token, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                &tok.kind, expected_token,
                "Test {:#?} - wrong 'kind'. Expected={:#?}, Got={:#?}",
                i, expected_token, tok.kind
            );

            assert_eq!(
                &tok.literal, expected_literal,
                "Test {} - wrong 'literal'. Expected={}, Got={}",
                i, expected_literal, tok.literal
            );
        }
    }
}
