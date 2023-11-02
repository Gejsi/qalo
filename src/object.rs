use std::fmt;

use thiserror::Error;

use crate::{
    ast::{ParserError, Statement},
    token::TokenKind,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    IntegerValue(i32),
    BooleanValue(bool),
    ReturnValue(Box<Object>),
    FunctionValue(Closure),
    UnitValue,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::IntegerValue(value) => write!(f, "{value}"),
            Object::BooleanValue(value) => write!(f, "{value}"),
            Object::ReturnValue(value) => value.fmt(f),
            Object::FunctionValue(value) => write!(f, "{value}"),
            Object::UnitValue => write!(f, "()"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Closure {
    pub parameters: Vec<String>,
    pub body: Statement,
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn({}) {}", self.parameters.join(", "), self.body)
    }
}

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Identifier not found: {0}")]
    IdentifierNotFound(String),

    #[error("Type mismatch: {0}")]
    TypeMismatch(String),

    #[error("Modulo of zero isn't allowed")]
    ModuloByZero,

    #[error("Division by zero isn't allowed")]
    DivisionByZero,

    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    #[error("Function call with the wrong number of arguments. Expected {0}, got {1}")]
    FunctionCallWrongArity(u8, u8),

    #[error("Return statement used outside a function")]
    ReturnOutsideFunction,

    #[error("Unsupported operator: {0}")]
    UnsupportedOperator(TokenKind),

    #[error("Parsing error: {0}")]
    ParsingError(#[from] ParserError),

    #[error("Unknown evaluation error")]
    Unknown,
}
