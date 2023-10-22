use std::{fmt, num::ParseIntError, rc::Rc};

use thiserror::Error;

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for statement in &self.0 {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Statement {
    VarStatement {
        kind: TokenKind,
        name: String,
        value: Expression,
    },

    ReturnStatement(Expression),

    ExpressionStatement(Expression),

    BlockStatement(Vec<Statement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::VarStatement { kind, name, value } => {
                write!(f, "{} {} = {};\n", kind, name, value)
            }
            Statement::ReturnStatement(expr) => write!(f, "return {};\n", expr),
            Statement::ExpressionStatement(expr) => write!(f, "{}\n", expr),
            Statement::BlockStatement(statements) => {
                write!(f, "{{\n")?;
                for statement in statements {
                    write!(f, "{}", statement)?;
                }
                write!(f, "}}\n")
            }
        }
    }
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

    GroupedExpression(Box<Expression>),

    CallExpression {
        path: String,
        arguments: Vec<CallExpressionArgument>,
    },

    // IfExpression {
    //     condition: Box<Expression>,
    // },
    Empty,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{}", s),
            Expression::IntegerLiteral(n) => write!(f, "{}", n),
            Expression::BooleanLiteral(b) => write!(f, "{}", b),
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", left, operator, right)
            }
            Expression::UnaryExpression { operator, value } => {
                write!(f, "({}{})", operator, value)
            }
            Expression::GroupedExpression(expr) => write!(f, "{}", expr),
            Expression::CallExpression { path, arguments } => {
                write!(f, "{}(", path)?;

                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", arg)?;
                }

                write!(f, ")")
            }
            Expression::Empty => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub struct CallExpressionArgument {
    pub name: String,
    pub value: Expression,
}

impl fmt::Display for CallExpressionArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
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
