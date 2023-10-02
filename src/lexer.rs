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

        // initial read to 'start' the lexer
        lexer.read_char();
        lexer
    }

    /// Give the next character.
    pub fn peek_char(&mut self) -> char {
        if self.next >= self.input.chars().count() {
            // reached EOF
            '\0'
        } else {
            self.input.chars().nth(self.next).unwrap_or('\0')
        }
    }

    /// Retrieve the next character and advance position in the input string.
    pub fn read_char(&mut self) {
        self.ch = self.peek_char();
        self.curr = self.next;
        self.next += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn read_identifier(&mut self) -> &str {
        let start = self.curr;

        while self.ch.is_alphanumeric() {
            self.read_char();
        }

        &self.input[start..self.curr]
    }

    pub fn read_number(&mut self) -> &str {
        let start = self.curr;

        while self.ch.is_digit(10) {
            self.read_char();
        }

        &self.input[start..self.curr]
    }

    /// Retrieve the current token and advance position in the input string.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

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
            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    let kind = TokenKind::lookup_identifier(&literal);

                    return Token {
                        kind,
                        literal: literal.to_string(),
                    };
                } else if self.ch.is_digit(10) {
                    let literal = self.read_number().to_string();

                    return Token {
                        kind: TokenKind::Int,
                        literal,
                    };
                } else {
                    Token {
                        kind: TokenKind::Illegal,
                        literal: self.ch.to_string(),
                    }
                }
            }
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

    #[test]
    fn next_token_advanced() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        "#;

        let tests = vec![
            (TokenKind::Let, "let"),
            (TokenKind::Identifier, "five"),
            (TokenKind::Assign, "="),
            (TokenKind::Int, "5"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Let, "let"),
            (TokenKind::Identifier, "ten"),
            (TokenKind::Assign, "="),
            (TokenKind::Int, "10"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Let, "let"),
            (TokenKind::Identifier, "add"),
            (TokenKind::Assign, "="),
            (TokenKind::Function, "fn"),
            (TokenKind::LeftParen, "("),
            (TokenKind::Identifier, "x"),
            (TokenKind::Comma, ","),
            (TokenKind::Identifier, "y"),
            (TokenKind::RightParen, ")"),
            (TokenKind::LeftBrace, "{"),
            (TokenKind::Identifier, "x"),
            (TokenKind::Plus, "+"),
            (TokenKind::Identifier, "y"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::RightBrace, "}"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Let, "let"),
            (TokenKind::Identifier, "result"),
            (TokenKind::Assign, "="),
            (TokenKind::Identifier, "add"),
            (TokenKind::LeftParen, "("),
            (TokenKind::Identifier, "five"),
            (TokenKind::Comma, ","),
            (TokenKind::Identifier, "ten"),
            (TokenKind::RightParen, ")"),
            (TokenKind::Semicolon, ";"),
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
