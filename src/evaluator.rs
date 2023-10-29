use std::{collections::HashMap, rc::Rc};

use crate::{
    ast::{Expression, Statement},
    environment::Environment,
    object::{EvalError, Object},
    parser::Parser,
    token::TokenKind,
};

#[derive(Debug)]
pub struct Evaluator<'a> {
    parser: Parser<'a>,
    env: Environment,
}

impl<'a> Evaluator<'a> {
    pub fn new(input: &'a str) -> Self {
        let parser = Parser::new(&input);
        let env = Environment::new();

        Evaluator { parser, env }
    }

    pub fn eval_program(&mut self) -> Result<Vec<Object>, EvalError> {
        let program = self.parser.parse_program()?;
        let mut objects: Vec<Object> = vec![];

        for statement in program.0 {
            if let Some(eval_statement) = self.eval_statement(statement)? {
                objects.push(eval_statement);
            }
        }

        Ok(objects)
    }

    // Most statements aren't evaluated, only *expression statements* are.
    fn eval_statement(&mut self, statement: Statement) -> Result<Option<Object>, EvalError> {
        let try_obj = match statement {
            Statement::VarStatement {
                kind: _,
                name,
                value,
            } => {
                let obj = self.eval_expression(value)?;
                self.env.set(name, obj);
                None
            }
            Statement::ReturnStatement(expr) => todo!(),
            Statement::ExpressionStatement(expr) => Some(self.eval_expression(expr)?),
            Statement::BlockStatement(statements) => {
                // Create a new environment linked to the current outer environment
                let inner_env = Environment {
                    store: HashMap::new(),
                    outer: Some(self.env.clone().into()),
                };

                let outer_env = std::mem::replace(&mut self.env, inner_env);

                for statement in statements {
                    self.eval_statement(statement)?;
                }

                self.env = outer_env;

                None
            }
        };

        Ok(try_obj)
    }

    fn eval_expression(&mut self, expr: Expression) -> Result<Object, EvalError> {
        let obj = match expr {
            Expression::IntegerLiteral(lit) => Object::Integer(lit),
            Expression::BooleanLiteral(lit) => Object::Boolean(lit),
            Expression::Identifier(name) => self.env.get(&name)?,
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => self.eval_binary_expression(left, operator, right)?,
            Expression::UnaryExpression { operator, value } => {
                self.eval_unary_expression(operator, value)?
            }
            Expression::GroupedExpression(expr) => self.eval_expression(*expr)?,
            Expression::CallExpression { path, arguments } => todo!(),
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => todo!(),
            Expression::FunctionExpression { parameters, body } => todo!(),
        };

        Ok(obj)
    }

    fn eval_binary_expression(
        &mut self,
        left: Box<Expression>,
        operator: TokenKind,
        right: Box<Expression>,
    ) -> Result<Object, EvalError> {
        let left_eval = self.eval_expression(*left)?;
        let right_eval = self.eval_expression(*right)?;

        let obj = match (left_eval, right_eval) {
            (Object::Integer(left_value), Object::Integer(right_value)) => match operator {
                TokenKind::Plus => Object::Integer(left_value + right_value),
                TokenKind::Minus => Object::Integer(left_value - right_value),
                TokenKind::Asterisk => Object::Integer(left_value * right_value),
                TokenKind::Equal => Object::Boolean(left_value == right_value),
                TokenKind::NotEqual => Object::Boolean(left_value != right_value),
                TokenKind::LessThan => Object::Boolean(left_value < right_value),
                TokenKind::GreaterThan => Object::Boolean(left_value > right_value),
                TokenKind::LessThanEqual => Object::Boolean(left_value <= right_value),
                TokenKind::GreaterThanEqual => Object::Boolean(left_value >= right_value),
                TokenKind::Modulus => {
                    if right_value == 0 {
                        return Err(EvalError::ModulusByZero);
                    } else {
                        Object::Integer(left_value % right_value)
                    }
                }
                TokenKind::Slash => {
                    if right_value == 0 {
                        return Err(EvalError::DivisionByZero);
                    } else {
                        Object::Integer(left_value / right_value)
                    }
                }
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            (Object::Boolean(left_value), Object::Boolean(right_value)) => match operator {
                TokenKind::Equal => Object::Boolean(left_value == right_value),
                TokenKind::NotEqual => Object::Boolean(left_value != right_value),
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            (left_value, right_value) => {
                return Err(EvalError::TypeMismatch(format!(
                "Cannot perform operation '{operator}' between '{left_value}' and '{right_value}'",
            )))
            }
        };

        Ok(obj)
    }

    fn eval_unary_expression(
        &mut self,
        operator: TokenKind,
        value: Box<Expression>,
    ) -> Result<Object, EvalError> {
        let obj = match operator {
            TokenKind::Bang => match self.eval_expression(*value)? {
                Object::Integer(lit) => {
                    if lit == 0 {
                        Object::Boolean(true)
                    } else {
                        Object::Boolean(false)
                    }
                }
                Object::Boolean(lit) => {
                    if lit == true {
                        Object::Boolean(false)
                    } else {
                        Object::Boolean(true)
                    }
                }
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            TokenKind::Minus => match self.eval_expression(*value)? {
                Object::Integer(lit) => Object::Integer(-lit),
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            _ => return Err(EvalError::UnsupportedOperator(operator)),
        };

        Ok(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_integer_literal() {
        let input = "5";
        let mut evaluator = Evaluator::new(&input);
        evaluator.eval_program().unwrap();
    }

    #[test]
    fn eval_boolean_literal() {
        let input = "true";
        let mut evaluator = Evaluator::new(&input);
        evaluator.eval_program().unwrap();
    }

    #[test]
    fn eval_boolean_expressions() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

        for (input, expected) in tests {
            let mut evaluator = Evaluator::new(input);
            let result = &evaluator.eval_program().unwrap()[0];

            let expected_obj = match expected {
                true => &Object::Boolean(true),
                false => &Object::Boolean(false),
            };

            assert_eq!(result, expected_obj);
        }
    }

    #[test]
    fn eval_binary_expressions() {
        let tests = vec![
            ("2 + 3", &Object::Integer(5)),
            ("4 - 1", &Object::Integer(3)),
            ("5 * 6", &Object::Integer(30)),
            ("10 / 2", &Object::Integer(5)),
            ("7 == 7", &Object::Boolean(true)),
            ("8 != 9", &Object::Boolean(true)),
            ("true == true", &Object::Boolean(true)),
            ("false != true", &Object::Boolean(true)),
        ];

        for (input, expected) in tests {
            let mut evaluator = Evaluator::new(input);
            let result = &evaluator.eval_program().unwrap()[0];
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn eval_unary_expressions() {
        let tests = vec![
            ("-2", &Object::Integer(-2)),
            ("!true", &Object::Boolean(false)),
            ("!false", &Object::Boolean(true)),
            ("!5", &Object::Boolean(false)),
            ("!!5", &Object::Boolean(true)),
            ("!0", &Object::Boolean(true)),
            ("!!true", &Object::Boolean(true)),
            ("!!false", &Object::Boolean(false)),
        ];

        for (input, expected) in tests {
            let mut evaluator = Evaluator::new(input);
            let result = &evaluator.eval_program().unwrap()[0];
            assert_eq!(result, expected);
        }
    }
}
