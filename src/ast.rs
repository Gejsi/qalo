use std::{num::ParseIntError, rc::Rc};

use thiserror::Error;

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug)]
pub enum Statement {
    VarStatement {
        kind: TokenKind,
        name: String,
        value: Expression,
    },

    ReturnStatement(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),

    IntegerLiteral(i32),

    BooleanLiteral(bool),

    BinaryExpression {
        left: Box<Expression>,
        operator: TokenKind,
        right: Box<Expression>,
    },

    UnaryExpression {
        operator: TokenKind,
        value: Box<Expression>,
    },

    ParenthesizedExpression(Box<Expression>),

    Empty,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Syntax error: {0}")]
    SyntaxError(String), // Describes a syntax error with an error message

    #[error("Unexpected token: {0:#?}")]
    UnexpectedToken(Rc<Token>), // Describes an unexpected token encountered during parsing

    #[error("Operator received an invalid operand type: {0:#?}")]
    InvalidOperandType(Rc<Token>), // Describes an error when an operator receives an invalid operand type

    #[error("Semantic error: {0}")]
    SemanticError(String),

    #[error("Failed to convert number to a 32 bit integer: {0}")]
    IntConversionError(#[from] ParseIntError),

    #[error("Unknown parsing error")]
    Unknown,
}
