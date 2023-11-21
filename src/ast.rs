use std::{
    collections::HashMap,
    fmt,
    num::{ParseIntError, TryFromIntError},
    rc::Rc,
};

use thiserror::Error;

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for statement in &self.0 {
            write!(f, "{statement}")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    // TODO: support different types of var statements
    VarStatement {
        kind: TokenKind,
        name: String,
        value: Expression,
    },

    // TODO: make the expression optional
    ReturnStatement(Expression),

    AssignStatement {
        name: String,
        value: Expression,
    },

    ExpressionStatement(Expression),

    BlockStatement(Vec<Statement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::VarStatement { kind, name, value } => {
                write!(f, "{} {} = {};", kind, name, value)
            }
            Statement::ReturnStatement(expr) => write!(f, "return {expr};"),
            Statement::AssignStatement { name, value } => write!(f, "{name} = {value};"),
            Statement::ExpressionStatement(expr) => write!(f, "{expr}"),
            Statement::BlockStatement(statements) => {
                write!(f, "{{")?;
                for statement in statements {
                    write!(f, "{}", statement)?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Identifier(String),

    IntegerLiteral(i32),

    BooleanLiteral(bool),

    StringLiteral(String),

    ArrayLiteral(Vec<Expression>),

    // TODO: support different types of keys, as long as they are hashable.
    MapLiteral(HashMap<String, Expression>),

    BinaryExpression {
        left: Box<Expression>,
        operator: TokenKind,
        right: Box<Expression>,
    },

    UnaryExpression {
        operator: TokenKind,
        value: Box<Expression>,
    },

    IndexExpression {
        value: Box<Expression>,
        index: Box<Expression>,
    },

    GroupedExpression(Box<Expression>),

    CallExpression {
        path: Box<Expression>,
        arguments: Vec<Expression>,
    },

    IfExpression {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },

    FunctionExpression {
        parameters: Vec<String>,
        body: Box<Statement>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{s}"),
            Expression::IntegerLiteral(n) => write!(f, "{n}"),
            Expression::BooleanLiteral(b) => write!(f, "{b}"),
            Expression::StringLiteral(s) => write!(f, "\"{s}\""),
            Expression::ArrayLiteral(elements) => {
                write!(f, "[")?;
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{element}")?;
                }
                write!(f, "]")
            }
            Expression::MapLiteral(map) => {
                write!(f, "{{")?;
                for (i, (key, value)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{key}\": {value}")?;
                }
                write!(f, "}}")
            }
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                write!(f, "({left} {operator} {right})")
            }
            Expression::UnaryExpression { operator, value } => {
                write!(f, "({operator}{value})")
            }
            Expression::IndexExpression { value, index } => {
                write!(f, "({value}[{index}])")
            }
            Expression::GroupedExpression(expr) => write!(f, "{expr}"),
            Expression::CallExpression { path, arguments } => {
                write!(f, "{path}(")?;

                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{arg}")?;
                }

                write!(f, ")")
            }

            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                if let Some(alternative) = alternative {
                    write!(f, "if {} {} else {}", condition, consequence, alternative)
                } else {
                    write!(f, "if {} {}", condition, consequence)
                }
            }

            Expression::FunctionExpression { parameters, body } => {
                write!(f, "fn(")?;
                for (i, param) in parameters.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if i < parameters.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ") {}", body)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Unexpected token: {0:#?}")]
    UnexpectedToken(Rc<Token>),

    #[error("Operator received an invalid operand type: {0:#?}")]
    InvalidOperandType(Rc<Token>),

    #[error("Failed to parse to a 32 bit integer: {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("Integral type conversion failed: {0}")]
    IntConversionError(#[from] TryFromIntError),

    #[error("Unknown parsing error")]
    Unknown,
}
