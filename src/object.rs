use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use thiserror::Error;

use crate::{
    ast::{ParserError, Statement},
    environment::Environment,
    token::TokenKind,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    IntegerValue(i32),
    BooleanValue(bool),
    StringValue(String),
    ArrayValue(Vec<Object>),
    MapValue(HashMap<String, Object>),
    ReturnValue(Box<Object>),
    FunctionValue(Closure),
    BuiltinValue(BuiltinFunction),
    UnitValue,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::IntegerValue(value) => write!(f, "{value}"),
            Object::BooleanValue(value) => write!(f, "{value}"),
            Object::StringValue(value) => write!(f, "\"{value}\""),
            Object::ArrayValue(elements) => {
                write!(f, "[")?;
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{element}")?;
                }
                write!(f, "]")
            }
            Object::MapValue(map) => {
                write!(f, "{{")?;
                for (i, (key, value)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{key}\": {value}")?;
                }
                write!(f, "}}")
            }
            Object::FunctionValue(value) => write!(f, "{value}"),
            Object::ReturnValue(value) => write!(f, "return {value}"),
            Object::BuiltinValue(value) => write!(f, "built-in function {value}"),
            Object::UnitValue => write!(f, "()"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Closure {
    pub parameters: Vec<String>,
    pub body: Statement,
    pub env: Rc<RefCell<Environment>>,
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn({}) {}", self.parameters.join(", "), self.body)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BuiltinFunction {
    Len,
    Append,
    Rest,
    Println,
    Print,
}

impl BuiltinFunction {
    /// Matches built-in functions.
    pub fn lookup_function(identifier: &str) -> Result<Object, EvalError> {
        match identifier {
            "len" => Ok(Object::BuiltinValue(BuiltinFunction::Len)),
            "append" => Ok(Object::BuiltinValue(BuiltinFunction::Append)),
            "rest" => Ok(Object::BuiltinValue(BuiltinFunction::Rest)),
            "println" => Ok(Object::BuiltinValue(BuiltinFunction::Println)),
            "print" => Ok(Object::BuiltinValue(BuiltinFunction::Print)),
            _ => Err(EvalError::IdentifierNotFound(identifier.to_owned())),
        }
    }
}

impl fmt::Display for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuiltinFunction::Len => write!(f, "len"),
            BuiltinFunction::Append => write!(f, "push"),
            BuiltinFunction::Rest => write!(f, "rest"),
            BuiltinFunction::Println => write!(f, "println"),
            BuiltinFunction::Print => write!(f, "print"),
        }
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

    #[error("Return statement used outside an expression")]
    ReturnOutsideExpression,

    #[error("Unsupported operator: {0}")]
    UnsupportedOperator(TokenKind),

    #[error("Parsing error: {0}")]
    ParsingError(#[from] ParserError),

    #[error("Unsupported argument type for built-in function: {0}")]
    UnsupportedArgumentType(String),

    #[error("Only arrays can be accessed through the index operator")]
    InvalidIndexUsage,

    #[error("This structure cannot be accessed with such type.")]
    InvalidIndexType,

    #[error("This structure has {0} elements but the index {1} is out of bounds.")]
    IndexOutOfBounds(usize, usize),

    #[error("This map doesn't have a value defined at key {0}")]
    ValueNotFound(String),
}
