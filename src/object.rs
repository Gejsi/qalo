use thiserror::Error;

use crate::{ast::ParserError, token::TokenKind};

// TODO: implement Display trait
#[derive(Debug)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
}

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    #[error("Type mismatch: {0}")]
    TypeMismatch(String),

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    #[error("Function call with the wrong number of arguments: {0}")]
    FunctionCallWrongArity(String),

    #[error("Return statement used outside a function")]
    ReturnOutsideFunction,

    #[error("Unsupported operator: {0}")]
    UnsupportedOperator(TokenKind),

    #[error("Parsing error: {0}")]
    ParsingError(#[from] ParserError),

    #[error("Unknown evaluation error")]
    Unknown,
}
