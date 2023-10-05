use thiserror::Error;

use crate::{
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Program<'a>(pub Vec<Statement<'a>>);

#[derive(Debug)]
pub enum Statement<'a> {
    VarStatement {
        kind: &'a TokenKind,
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
    UnexpectedToken(Token), // Describes an unexpected token encountered during parsing
    #[error("Operator received an invalid operand type: {0:#?}")]
    InvalidOperandType(Token), // Describes an error when an operator receives an invalid operand type
    #[error("Input ends unexpectedly")]
    UnexpectedEndOfInput, // Describes an error when the input ends
    #[error("Semantic error: {0}")]
    SemanticError(String),
    #[error("Unknown parsing error")]
    Unknown,
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub cur: Token,
    pub next: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(&input);

        let mut parser = Self {
            lexer,
            cur: Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
            next: Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
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
        self.cur = std::mem::replace(&mut self.next, self.lexer.next_token());
    }

    pub fn expect_token(&mut self, token_kind: &TokenKind) -> Result<(), ParserError> {
        if &self.next.kind != token_kind {
            return Err(ParserError::UnexpectedToken(self.cur.clone()));
        }

        self.eat_token();
        Ok(())
    }

    pub fn parse_var_statement(&mut self) -> Result<Statement, ParserError> {
        // let kind = if self.cur.kind != TokenKind::Let {
        //     return Err(ParserError::SyntaxError(
        //         "Binding statements must start with `let`".to_string(),
        //     ));
        // } else {
        //     &self.cur.kind
        // };

        // self.expect_token(&TokenKind::Identifier)?;

        Ok(Statement::VarStatement {
            kind: &TokenKind::Let,
            name: Identifier("aa".to_string()),
            value: Expression::IntegerLiteral(1),
        })
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        Ok(Program(vec![]))
    }
}
