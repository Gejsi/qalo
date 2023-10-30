use std::fmt;

use thiserror::Error;

use crate::{
    ast::{ParserError, Statement},
    environment::Environment,
    token::TokenKind,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Return(Box<Object>),
    Function(Closure),
    Unit,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{value}"),
            Object::Boolean(value) => write!(f, "{value}"),
            Object::Return(value) => value.fmt(f),
            Object::Function(value) => write!(f, "{value}"),
            Object::Unit => write!(f, "()"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Closure {
    pub parameters: Vec<String>,
    pub body: Statement,
    pub env: Environment,
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn({}) {{ {} }}", self.parameters.join(", "), self.body)
    }
}

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    #[error("Type mismatch: {0}")]
    TypeMismatch(String),

    #[error("Modulus of zero isn't allowed")]
    ModulusByZero,

    #[error("Division by zero isn't allowed")]
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
