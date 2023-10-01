use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    curr: usize, // current position in input (points to current char)
    next: usize, // next position in input (after current char)
    ch: char,    // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
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
                literal: "=",
            },
            '+' => Token {
                kind: TokenKind::Plus,
                literal: "+",
            },
            '(' => Token {
                kind: TokenKind::LeftParen,
                literal: "(",
            },
            ')' => Token {
                kind: TokenKind::RightParen,
                literal: ")",
            },
            '{' => Token {
                kind: TokenKind::LeftBrace,
                literal: "{",
            },
            '}' => Token {
                kind: TokenKind::RightBrace,
                literal: "}",
            },
            ';' => Token {
                kind: TokenKind::Semicolon,
                literal: ";",
            },
            ',' => Token {
                kind: TokenKind::Comma,
                literal: ",",
            },
            '\0' => Token {
                kind: TokenKind::Eof,
                literal: "",
            },
            _ => Token {
                kind: TokenKind::Illegal,
                literal: "",
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

        let mut lexer = Lexer::new(input);

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
