use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    /// Current position in input (points to current char)
    cur: usize,
    /// Next position in input (after current char)
    next: usize,
    /// Current char under examination
    ch: char,
}

const EOF_CHAR: char = '\0';

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            cur: 0,
            next: 0,
            ch: EOF_CHAR,
        };

        lexer.eat_char();

        lexer
    }

    /// Give the next character.
    pub fn peek_char(&mut self) -> char {
        if self.next >= self.input.chars().count() {
            // reached EOF
            EOF_CHAR
        } else {
            self.input.chars().nth(self.next).unwrap_or(EOF_CHAR)
        }
    }

    /// Retrieve the next character and advance position in the input string.
    pub fn eat_char(&mut self) {
        self.ch = self.peek_char();
        self.cur = self.next;
        self.next += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.eat_char();
        }
    }

    pub fn eat_identifier(&mut self) -> &str {
        let start = self.cur;

        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.eat_char();
        }

        &self.input[start..self.cur]
    }

    // TODO: add support for different types of numbers; only ints are supported currently.
    pub fn eat_number(&mut self) -> &str {
        let start = self.cur;

        while self.ch.is_digit(10) {
            self.eat_char();
        }

        &self.input[start..self.cur]
    }

    pub fn eat_string(&mut self) -> &str {
        let start = self.cur + 1;

        loop {
            self.eat_char();

            // TODO: add support for escape characters
            if self.ch == '"' || self.ch == EOF_CHAR {
                break;
            }
        }

        &self.input[start..self.cur]
    }

    /// Retrieve the current token and advance position in the input string.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.eat_char();
                    Token {
                        kind: TokenKind::Equal,
                        literal: "==".to_string(),
                    }
                } else {
                    Token {
                        kind: TokenKind::Assign,
                        literal: "=".to_string(),
                    }
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.eat_char();
                    Token {
                        kind: TokenKind::NotEqual,
                        literal: "!=".to_string(),
                    }
                } else {
                    Token {
                        kind: TokenKind::Bang,
                        literal: "!".to_string(),
                    }
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.eat_char();
                    Token {
                        kind: TokenKind::LessThanEqual,
                        literal: "<=".to_string(),
                    }
                } else {
                    Token {
                        kind: TokenKind::LessThan,
                        literal: "<".to_string(),
                    }
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.eat_char();
                    Token {
                        kind: TokenKind::GreaterThanEqual,
                        literal: ">=".to_string(),
                    }
                } else {
                    Token {
                        kind: TokenKind::GreaterThan,
                        literal: ">".to_string(),
                    }
                }
            }
            '+' => Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            '-' => Token {
                kind: TokenKind::Minus,
                literal: "-".to_string(),
            },
            '/' => Token {
                kind: TokenKind::Slash,
                literal: "/".to_string(),
            },
            '*' => Token {
                kind: TokenKind::Asterisk,
                literal: "*".to_string(),
            },
            '%' => Token {
                kind: TokenKind::Percentage,
                literal: "%".to_string(),
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
            '[' => Token {
                kind: TokenKind::LeftSquare,
                literal: "[".to_string(),
            },
            ']' => Token {
                kind: TokenKind::RightSquare,
                literal: "]".to_string(),
            },
            ':' => Token {
                kind: TokenKind::Colon,
                literal: ":".to_string(),
            },
            ';' => Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            ',' => Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            '"' => {
                let literal = self.eat_string().to_string();

                Token {
                    kind: TokenKind::String,
                    literal,
                }
            }
            EOF_CHAR => Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let literal = self.eat_identifier();
                    let kind = TokenKind::lookup_identifier(&literal);

                    return Token {
                        kind,
                        literal: literal.to_string(),
                    };
                } else if self.ch.is_digit(10) {
                    let literal = self.eat_number().to_string();

                    return Token {
                        kind: TokenKind::Integer,
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

        self.eat_char();

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    fn test_tokenization_iter(input: &str, tests: Vec<(TokenKind, &str)>) {
        let mut lexer = Lexer::new(input);

        for (i, (expected_token, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                &tok.kind, expected_token,
                "Test {} - wrong 'kind'. Expected={:#?}, Got={:#?}",
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

        test_tokenization_iter(input, tests)
    }

    #[test]
    fn next_token2() {
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
            (TokenKind::Integer, "5"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Let, "let"),
            (TokenKind::Identifier, "ten"),
            (TokenKind::Assign, "="),
            (TokenKind::Integer, "10"),
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

        test_tokenization_iter(input, tests)
    }

    #[test]
    fn next_token3() {
        let input = r##"
            "foo bar";
            [1, 2];
            {"foo": "bar"}
        "##;

        let tests = vec![
            (TokenKind::String, "foo bar"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::LeftSquare, "["),
            (TokenKind::Integer, "1"),
            (TokenKind::Comma, ","),
            (TokenKind::Integer, "2"),
            (TokenKind::RightSquare, "]"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::LeftBrace, "{"),
            (TokenKind::String, "foo"),
            (TokenKind::Colon, ":"),
            (TokenKind::String, "bar"),
            (TokenKind::RightBrace, "}"),
            (TokenKind::Eof, ""),
        ];

        test_tokenization_iter(input, tests)
    }
}
