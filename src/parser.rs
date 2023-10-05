use std::{num::ParseIntError, rc::Rc};

use thiserror::Error;

use crate::{
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug)]
pub enum Statement {
    VarStatement {
        kind: TokenKind,
        name: Identifier,
        value: Expression,
    },

    ReturnStatement(Expression),

    /// e.g. `a + b;`
    ExpressionStatement(Expression),
}

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(i32),
    BooleanLiteral(bool),
    InfixExpression {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Syntax error: {0}")]
    SyntaxError(String), // Describes a syntax error with an error message
    #[error("Unexpected token: {0:#?}")]
    UnexpectedToken(Rc<Token>), // Describes an unexpected token encountered during parsing
    #[error("Operator received an invalid operand type: {0:#?}")]
    InvalidOperandType(Rc<Token>), // Describes an error when an operator receives an invalid operand type
    #[error("Input ends unexpectedly")]
    UnexpectedEndOfInput, // Describes an error when the input ends
    #[error("Semantic error: {0}")]
    SemanticError(String),
    #[error("Failed to convert number to a 32 bit integer: {0}")]
    IntConversionError(#[from] ParseIntError),
    #[error("Unknown parsing error")]
    Unknown,
}

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
            name: Identifier(name.literal.clone()),
            value: expr,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.eat_token();

        let expr = match self.cur.kind {
            TokenKind::Int => match self.next.kind {
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
                _ => todo!(),
            },
            _ => todo!(),
        };

        Ok(expr)
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        Ok(Program(vec![]))
    }
}
