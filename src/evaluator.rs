use std::{cell::RefCell, rc::Rc};

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
    env: Rc<RefCell<Environment>>,
}

impl<'a> Evaluator<'a> {
    pub fn new(input: &'a str) -> Self {
        let parser = Parser::new(&input);
        let env = Rc::new(RefCell::new(Environment::new()));

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

    fn eval_statement(&mut self, statement: Statement) -> Result<Option<Object>, EvalError> {
        let try_obj = match statement {
            Statement::VarStatement {
                kind: _,
                name,
                value,
            } => {
                let obj = self.eval_expression(value)?;
                self.env.borrow_mut().set(name, obj);
                None
            }
            Statement::ReturnStatement(expr) => None,
            Statement::ExpressionStatement(expr) => Some(self.eval_expression(expr)?),
            Statement::BlockStatement(statements) => {
                // create a new environment linked to the current outer environment
                let mut inner_env = Environment::new();
                inner_env.outer = Some(self.env.clone());
                let outer_env = std::mem::replace(&mut self.env, Rc::new(RefCell::new(inner_env)));

                // save last evaluated object
                let mut obj: Option<Object> = None;
                for statement in statements {
                    if let Some(statement) = self.eval_statement(statement)? {
                        obj = Some(statement);
                    }
                }

                // go back to the outer environment
                self.env = outer_env;

                // return the last evaluated object
                obj
            }
        };

        Ok(try_obj)
    }

    fn eval_expression(&mut self, expr: Expression) -> Result<Object, EvalError> {
        let obj = match expr {
            Expression::IntegerLiteral(lit) => Object::Integer(lit),
            Expression::BooleanLiteral(lit) => Object::Boolean(lit),
            Expression::Identifier(name) => self.env.borrow().get(&name)?,
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
            } => self.eval_if_expression(condition, consequence, alternative)?,
            Expression::FunctionExpression { parameters, body } => {
                self.eval_function_expression(parameters, body)?
            }
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
                Object::Integer(lit) => Object::Integer(!lit),
                Object::Boolean(lit) => Object::Boolean(!lit),
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

    fn eval_if_expression(
        &mut self,
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    ) -> Result<Object, EvalError> {
        let obj = match self.eval_expression(*condition)? {
            Object::Boolean(true) => self.eval_statement(*consequence)?,
            Object::Boolean(false) => {
                if let Some(alt) = alternative {
                    self.eval_statement(*alt)?
                } else {
                    Some(Object::Unit)
                }
            }
            _ => {
                return Err(EvalError::TypeMismatch(
                    "Condition must be a boolean".to_string(),
                ))
            }
        };

        obj.map_or(Ok(Object::Unit), Ok)
    }

    fn eval_function_expression(
        &mut self,
        parameters: Vec<String>,
        body: Box<Statement>,
    ) -> Result<Object, EvalError> {
        todo!()
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
            let mut evaluator = Evaluator::new(&input);
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
            let mut evaluator = Evaluator::new(&input);
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
            ("!5", &Object::Integer(-6)),
            ("!!5", &Object::Integer(5)),
            ("!0", &Object::Integer(-1)),
            ("!!true", &Object::Boolean(true)),
            ("!!false", &Object::Boolean(false)),
        ];

        for (input, expected) in tests {
            let mut evaluator = Evaluator::new(&input);
            let result = &evaluator.eval_program().unwrap()[0];
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn eval_if_expression() {
        let input = r#"
            let a = if 2 > 1 {
                let b = 2;
                b + b;
            } else {
                3
            };

            a;

            let b = if 2 > 5 {
                2
            } else {
                3
            };

            b;
        "#;
        let mut evaluator = Evaluator::new(&input);
        let objects = evaluator.eval_program().unwrap();
        assert_eq!(objects[0], Object::Integer(4));
        assert_eq!(objects[1], Object::Integer(3));
    }

    #[test]
    fn eval_block_statement() {
        let input = r#"
            let a = 2;

            {
                let b = 3;
                b;

                {
                    a;
                }
            }

            a;
        "#;
        let mut evaluator = Evaluator::new(&input);
        evaluator.eval_program().unwrap();
    }
}
