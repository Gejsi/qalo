use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{Expression, Statement},
    environment::Environment,
    object::{Closure, EvalError, Object},
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
            objects.push(self.eval_statement(statement)?);
        }

        Ok(objects)
    }

    fn eval_statement(&mut self, statement: Statement) -> Result<Object, EvalError> {
        match statement {
            Statement::VarStatement {
                kind: _,
                name,
                value,
            } => {
                let obj = self.eval_expression(value)?;
                self.env.borrow_mut().set(name, obj);
                Ok(Object::UnitValue)
            }
            Statement::ReturnStatement(expr) => {
                let obj = self.eval_expression(expr)?;
                Ok(Object::ReturnValue(Box::new(obj)))
            }
            Statement::ExpressionStatement(expr) => Ok(self.eval_expression(expr)?),
            Statement::BlockStatement(statements) => {
                let inner_env = self.create_enclosed_env();
                let outer_env = std::mem::replace(&mut self.env, inner_env);

                // save last evaluated object
                let mut obj = Object::UnitValue;
                for statement in statements {
                    obj = self.eval_statement(statement)?;

                    // if the current object is a `return` value, stop evaluating this block.
                    if let Object::ReturnValue(_) = obj {
                        break;
                    }
                }

                // go back to the outer environment
                self.env = outer_env;

                // return the last evaluated object
                Ok(obj)
            }
        }
    }

    fn eval_expression(&mut self, expr: Expression) -> Result<Object, EvalError> {
        let obj = match expr {
            Expression::IntegerLiteral(lit) => Object::IntegerValue(lit),
            Expression::BooleanLiteral(lit) => Object::BooleanValue(lit),
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
            Expression::CallExpression { path, arguments } => {
                self.eval_call_expression(path, arguments)?
            }
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => self.eval_if_expression(condition, consequence, alternative)?,
            Expression::FunctionExpression { parameters, body } => {
                self.eval_function_expression(parameters, body)?
            }
        };

        // unwrap return values
        if let Object::ReturnValue(ref inner_obj) = obj {
            // FIX: this isn't enough to handle all cases
            if self.env.borrow().outer.is_none() {
                return Ok(*inner_obj.clone());
            }
        }

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
            (Object::IntegerValue(left_value), Object::IntegerValue(right_value)) => match operator
            {
                TokenKind::Plus => Object::IntegerValue(left_value + right_value),
                TokenKind::Minus => Object::IntegerValue(left_value - right_value),
                TokenKind::Asterisk => Object::IntegerValue(left_value * right_value),
                TokenKind::Equal => Object::BooleanValue(left_value == right_value),
                TokenKind::NotEqual => Object::BooleanValue(left_value != right_value),
                TokenKind::LessThan => Object::BooleanValue(left_value < right_value),
                TokenKind::GreaterThan => Object::BooleanValue(left_value > right_value),
                TokenKind::LessThanEqual => Object::BooleanValue(left_value <= right_value),
                TokenKind::GreaterThanEqual => Object::BooleanValue(left_value >= right_value),
                TokenKind::Percentage => {
                    if right_value == 0 {
                        return Err(EvalError::ModuloByZero);
                    } else {
                        Object::IntegerValue(left_value % right_value)
                    }
                }
                TokenKind::Slash => {
                    if right_value == 0 {
                        return Err(EvalError::DivisionByZero);
                    } else {
                        Object::IntegerValue(left_value / right_value)
                    }
                }
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            (Object::BooleanValue(left_value), Object::BooleanValue(right_value)) => match operator
            {
                TokenKind::Equal => Object::BooleanValue(left_value == right_value),
                TokenKind::NotEqual => Object::BooleanValue(left_value != right_value),
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            (left_value, right_value) => {
                return Err(EvalError::TypeMismatch(format!(
                "Cannot perform operation '{operator}' between '{left_value}' and '{right_value}'",
            )));
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
                Object::IntegerValue(lit) => Object::IntegerValue(!lit),
                Object::BooleanValue(lit) => Object::BooleanValue(!lit),
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            TokenKind::Minus => match self.eval_expression(*value)? {
                Object::IntegerValue(lit) => Object::IntegerValue(-lit),
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
            Object::BooleanValue(lit) => {
                if lit {
                    self.eval_statement(*consequence)?
                } else if let Some(alt) = alternative {
                    self.eval_statement(*alt)?
                } else {
                    Object::UnitValue
                }
            }
            _ => {
                return Err(EvalError::TypeMismatch(
                    "`if` condition must be a boolean".to_owned(),
                ))
            }
        };

        Ok(obj)
    }

    fn eval_function_expression(
        &mut self,
        parameters: Vec<String>,
        body: Box<Statement>,
    ) -> Result<Object, EvalError> {
        let closure = Closure {
            parameters,
            body: *body,
            env: self.create_enclosed_env(),
        };

        Ok(Object::FunctionValue(closure))
    }

    fn eval_call_expression(
        &mut self,
        path: String,
        arguments: Vec<Expression>,
    ) -> Result<Object, EvalError> {
        let function = self.env.borrow().get(&path)?;

        let obj = match function {
            Object::FunctionValue(Closure {
                parameters,
                body,
                env,
            }) => {
                if parameters.len() != arguments.len() {
                    return Err(EvalError::FunctionCallWrongArity(
                        parameters.len() as u8,
                        arguments.len() as u8,
                    ));
                }

                let outer_env = std::mem::replace(&mut self.env, env);

                let arguments = arguments
                    .into_iter()
                    .map(|arg| self.eval_expression(arg))
                    .collect::<Result<Vec<Object>, EvalError>>()?;

                for (param, arg) in parameters.into_iter().zip(arguments.into_iter()) {
                    self.env.borrow_mut().set(param, arg);
                }

                let body_obj = self.eval_statement(body)?;
                self.env = outer_env;
                body_obj
            }

            _ => {
                return Err(EvalError::FunctionNotFound(
                    "Check if this identifier is a declared function".to_owned(),
                ));
            }
        };

        Ok(obj)
    }

    /// Creates a new environment linked to the outer environment
    fn create_enclosed_env(&mut self) -> Rc<RefCell<Environment>> {
        let mut inner_env = Environment::new();
        inner_env.outer = Some(self.env.clone());
        Rc::new(RefCell::new(inner_env))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_integer_literal() {
        let input = "5";
        let mut evaluator = Evaluator::new(&input);
        let result = &evaluator.eval_program().unwrap()[0];
        assert_eq!(result, &Object::IntegerValue(5));
    }

    #[test]
    fn eval_boolean_literal() {
        let input = "true";
        let mut evaluator = Evaluator::new(&input);
        let result = &evaluator.eval_program().unwrap()[0];
        assert_eq!(result, &Object::BooleanValue(true));
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
                true => &Object::BooleanValue(true),
                false => &Object::BooleanValue(false),
            };

            assert_eq!(result, expected_obj);
        }
    }

    #[test]
    fn eval_binary_expressions() {
        let tests = vec![
            ("2 + 3", &Object::IntegerValue(5)),
            ("4 - 1", &Object::IntegerValue(3)),
            ("5 * 6", &Object::IntegerValue(30)),
            ("10 / 2", &Object::IntegerValue(5)),
            ("7 == 7", &Object::BooleanValue(true)),
            ("8 != 9", &Object::BooleanValue(true)),
            ("true == true", &Object::BooleanValue(true)),
            ("false != true", &Object::BooleanValue(true)),
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
            ("-2", &Object::IntegerValue(-2)),
            ("!true", &Object::BooleanValue(false)),
            ("!false", &Object::BooleanValue(true)),
            ("!5", &Object::IntegerValue(-6)),
            ("!!5", &Object::IntegerValue(5)),
            ("!0", &Object::IntegerValue(-1)),
            ("!!true", &Object::BooleanValue(true)),
            ("!!false", &Object::BooleanValue(false)),
        ];

        for (input, expected) in tests {
            let mut evaluator = Evaluator::new(&input);
            let result = &evaluator.eval_program().unwrap()[0];
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn eval_if_expression() {
        let tests = vec![
            ("if true { 10 }", &Object::IntegerValue(10)),
            ("if false { 10 }", &Object::UnitValue),
            ("if 1 < 2 { 10 }", &Object::IntegerValue(10)),
            ("if 1 > 2 { 10 }", &Object::UnitValue),
            ("if 1 > 2 { 10 } else { 20 }", &Object::IntegerValue(20)),
            ("if 1 < 2 { 10 } else { 20 }", &Object::IntegerValue(10)),
        ];

        for (input, expected) in tests {
            let mut evaluator = Evaluator::new(&input);
            let result = &evaluator.eval_program().unwrap()[0];
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn eval_function_expression() {
        let input = r#"
            let foo = fn(x) {
                let double = fn(y) { y * 2; };
                return double(x);
            };

            let bar = foo(3);
            bar;
        "#;
        let mut evaluator = Evaluator::new(&input);
        let result = &evaluator.eval_program().unwrap()[2];
        assert_eq!(result, &Object::IntegerValue(6));
    }

    #[test]
    fn eval_function_expressions() {
        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
            // ("fn(x) { x; }(5)", 5),
        ];

        for (input, expected) in tests {
            let mut evaluator = Evaluator::new(&input);
            let result = &evaluator.eval_program().unwrap()[1];
            let expected_obj = &Object::IntegerValue(expected);
            assert_eq!(result, expected_obj);
        }
    }

    #[test]
    fn eval_block_statement() {
        let input = r#"
            let a = 2;

            {
                let b = 3;
                b;
            }

            a;
        "#;
        let mut evaluator = Evaluator::new(&input);
        let result = &evaluator.eval_program().unwrap()[2];
        assert_eq!(result, &Object::IntegerValue(2));
    }

    #[test]
    fn eval_static_scope() {
        let input = r#"
            let i = 5;
            let foo = fn(i) {
                i;
            };

            foo(10);
            i;
        "#;
        let mut evaluator = Evaluator::new(&input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(&result[2], &Object::IntegerValue(10));
        assert_eq!(&result[3], &Object::IntegerValue(5));
    }

    #[test]
    fn eval_closure() {
        let input = r#"
            let newAdder = fn(x) {
                fn(y) { x + y };
            };

            let addTwo = newAdder(2);
            addTwo(2);
        "#;
        let mut evaluator = Evaluator::new(&input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(&result[2], &Object::IntegerValue(4));
    }

    // #[test]
    // fn eval_nested_returns() {
    //     let input = r#"
    //         let bar = fn() { return 2; };
    //         let baz = if true { 2; };

    //         let foo = if bar() + 1 == 3 {
    //             if true {
    //                 {
    //                     return fn(x) { x; };
    //                 }
    //             }

    //             return 1;
    //         };

    //         let id = foo(3);
    //         id;
    //     "#;
    //     let mut evaluator = Evaluator::new(&input);
    //     evaluator.eval_program().unwrap();
    // }
}
