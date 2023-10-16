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

/// The resulting tuple represents the binding power
/// of an operator. For example:
/// a   +   b   *   c   *   d   +   e
///. 1   2   3   4   3   4   1   2
#[derive(Debug)]
pub struct Precedence(u8, u8);

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
            self.next = self.lexer.next_token().into();
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

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut statements: Vec<Statement> = vec![];

        while self.cur.kind != TokenKind::Eof {
            statements.push(self.parse_statement()?);
            self.eat_token();
        }

        Ok(Program(statements))
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.cur.kind {
            TokenKind::Let => self.parse_var_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => todo!(),
        }
    }

    pub fn parse_var_statement(&mut self) -> Result<Statement, ParserError> {
        let kind = if self.cur.kind != TokenKind::Let {
            return Err(ParserError::SyntaxError(
                "Binding statements must start with `let`".to_string(),
            ));
        } else {
            self.cur.kind.clone()
        };

        let name = self.expect_token(TokenKind::Identifier)?;
        self.expect_token(TokenKind::Assign)?;
        let expr = self.parse_expression(0)?;
        self.expect_token(TokenKind::Semicolon)?;

        Ok(Statement::VarStatement {
            kind,
            name: name.literal.clone(),
            value: expr,
        })
    }

    pub fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        if self.cur.kind != TokenKind::Return {
            return Err(ParserError::SyntaxError(
                "Return statements must start with `return`".to_string(),
            ));
        }

        let expr = self.parse_expression(0)?;
        self.expect_token(TokenKind::Semicolon)?;
        Ok(Statement::ReturnStatement(expr))
    }

    fn infix_precedence(op: &TokenKind) -> Option<Precedence> {
        match op {
            TokenKind::Plus | TokenKind::Minus => Some(Precedence(1, 2)),
            TokenKind::Asterisk | TokenKind::Slash => Some(Precedence(3, 4)),
            _ => None,
        }
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<Expression, ParserError> {
        self.eat_token();
        let mut expr = match self.cur.kind {
            TokenKind::Integer => Expression::IntegerLiteral(self.cur.literal.parse::<i32>()?),
            TokenKind::Identifier => Expression::Identifier(self.cur.literal.clone()),
            TokenKind::True => Expression::BooleanLiteral(true),
            TokenKind::False => Expression::BooleanLiteral(false),
            // TokenKind::LeftParen => {
            //     self.eat_token();
            //     let expr = self.parse_expression()?;
            //     self.expect_token(TokenKind::RightParen)?;
            //     Expression::ParenthesizedExpression(Box::new(expr))
            // }
            _ => {
                return Err(ParserError::UnexpectedToken(self.cur.clone()));
            }
        };

        // Pratt parsing uses both a loop and recursion to
        // handle grouping based on precedences.
        loop {
            // TODO: handle prefix parsing, similar to the `if` below

            // parse binary expressions based on infix operators precedences
            if let Some(Precedence(left_prec, right_prec)) = Self::infix_precedence(&self.next.kind)
            {
                if left_prec < min_prec {
                    break;
                }

                self.eat_token();
                let operator = self.cur.kind.clone();

                expr = match self.cur.kind {
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Asterisk => {
                        let right = self.parse_expression(right_prec)?;

                        Expression::BinaryExpression {
                            left: Box::new(expr),
                            operator,
                            right: Box::new(right),
                        }
                    }
                    _ => {
                        return Err(ParserError::UnexpectedToken(self.cur.clone()));
                    }
                };

                continue;
            }

            break;
        }

        Ok(expr)
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

        let num_vars = input.lines().count() - 2;
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

    #[test]
    fn parse_program() {
        let input = r#"
            let a = 1;
            let b = a + 1;
            return a / b;
        "#;

        let mut parser = Parser::new(&input);
        parser.parse_program().unwrap();
    }
}
