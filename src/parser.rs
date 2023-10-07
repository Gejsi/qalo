use std::rc::Rc;

use crate::{
    ast::{Expression, ParserError, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub cur: Rc<Token>,
    pub next: Rc<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(&input);

        let mut parser = Self {
            lexer,
            cur: Rc::new(Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            }),
            next: Rc::new(Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            }),
        };

        // consume two tokens to set `cur` and `next` correctly
        parser.eat_token();
        parser.eat_token();

        parser
    }

    pub fn eat_token(&mut self) {
        /*
            This is like doing...
            ```
            self.cur = self.next;
            self.next = self.lexer.next_token();
            ```
            ... but respecting the borrow checker.
        */
        self.cur = std::mem::replace(&mut self.next, self.lexer.next_token().into());
    }

    pub fn expect_token(&mut self, token_kind: TokenKind) -> Result<Rc<Token>, ParserError> {
        if self.next.kind != token_kind {
            return Err(ParserError::UnexpectedToken(self.next.clone()));
        }

        self.eat_token();
        Ok(self.cur.clone())
    }

    pub fn parse_var_statement(&mut self) -> Result<Statement, ParserError> {
        let kind = if self.cur.kind != TokenKind::Let {
            println!("{:?}", self.cur);
            return Err(ParserError::SyntaxError(
                "Binding statements must start with `let`".to_string(),
            ));
        } else {
            self.cur.kind.clone()
        };

        let name = self.expect_token(TokenKind::Identifier)?;

        self.expect_token(TokenKind::Assign)?;

        let expr = self.parse_expression()?;

        self.expect_token(TokenKind::Semicolon)?;

        Ok(Statement::VarStatement {
            kind,
            name: name.literal.clone(),
            value: expr,
        })
    }

    pub fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        if self.cur.kind != TokenKind::Return {
            println!("{:?}", self.cur);
            return Err(ParserError::SyntaxError(
                "Return statements must start with `return`".to_string(),
            ));
        }

        let expr = self.parse_expression()?;
        self.expect_token(TokenKind::Semicolon)?;
        Ok(Statement::ReturnStatement(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.eat_token();

        let expr = match self.cur.kind {
            TokenKind::Integer => match self.next.kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Asterisk => {
                    let left =
                        Box::new(Expression::IntegerLiteral(self.cur.literal.parse::<i32>()?));

                    self.eat_token();
                    let operator = self.cur.literal.clone();

                    let right = Box::new(self.parse_expression()?);

                    Expression::InfixExpression {
                        left,
                        operator,
                        right,
                    }
                }
                TokenKind::Semicolon => {
                    Expression::IntegerLiteral(self.cur.literal.parse::<i32>()?)
                }
                _ => {
                    return Err(ParserError::UnexpectedToken(self.next.clone()));
                }
            },

            TokenKind::True => Expression::BooleanLiteral(true),
            TokenKind::False => Expression::BooleanLiteral(false),

            TokenKind::Identifier => match self.next.kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Asterisk => {
                    let left = Box::new(Expression::Identifier(self.cur.literal.clone()));

                    self.eat_token();
                    let operator = self.cur.literal.clone();

                    let right = Box::new(self.parse_expression()?);

                    Expression::InfixExpression {
                        left,
                        operator,
                        right,
                    }
                }
                _ => Expression::Identifier(self.cur.literal.clone()),
            },

            _ => {
                return Err(ParserError::UnexpectedToken(self.cur.clone()));
            }
        };

        Ok(expr)
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        Ok(Program(vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_var_statement() {
        let input = r#"
            let five = 5;
            let taken = false;
            let temp = taken;
            let seven = five + 2 * 1;
        "#;

        let num_vars = 4;
        let mut parser = Parser::new(&input);

        (0..num_vars).for_each(|_| {
            parser.parse_var_statement().unwrap();
            parser.eat_token();
        });
    }

    #[test]
    fn parse_return_statement() {
        let input = r#"
            return token;
        "#;

        let mut parser = Parser::new(&input);
        parser.parse_return_statement().unwrap();
    }
}
