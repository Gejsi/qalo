use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    ast::{Expression, ParserError, Statement},
    environment::Environment,
    object::{BuiltinFunction, Closure, EvalError, Object},
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
        let parser = Parser::new(input);
        let env = Rc::new(RefCell::new(Environment::default()));

        Evaluator { parser, env }
    }

    pub fn eval_program(&mut self) -> Result<Vec<Object>, EvalError> {
        let program = self.parser.parse_program()?;
        let mut objects: Vec<Object> = vec![];

        for statement in program.0 {
            let obj = self.eval_statement(statement)?;

            // unwrap top-level return values
            if let Object::ReturnValue(inner_obj) = obj {
                objects.push(*inner_obj);
            } else {
                objects.push(obj);
            }
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
                let obj = self.eval_expression(value, true)?;
                self.env.borrow_mut().set(name, obj);
                Ok(Object::UnitValue)
            }
            Statement::ReturnStatement(_) => {
                // return statements aren't allowed at the top-level scope
                return Err(EvalError::ReturnOutsideExpression);
            }
            Statement::AssignStatement { name, value } => {
                let obj = self.eval_expression(value, true)?;
                self.env.borrow_mut().set(name, obj);
                Ok(Object::UnitValue)
            }
            Statement::ExpressionStatement(expr) => Ok(self.eval_expression(expr, true)?),
            Statement::BlockStatement(statements) => {
                let inner_env = self.create_enclosed_env();
                let outer_env = std::mem::replace(&mut self.env, inner_env);

                // save last evaluated object
                let mut obj = Object::UnitValue;

                for statement in statements {
                    // handle return statements inside a block
                    if let Statement::ReturnStatement(expr) = statement {
                        let expr_eval = if let Some(expr) = expr {
                            self.eval_expression(expr, true)?
                        } else {
                            Object::UnitValue
                        };

                        // if the result of the evaluation is a *return value*, keep it to
                        // propagate it to upper blocks...
                        if matches!(expr_eval, Object::ReturnValue(_)) {
                            obj = expr_eval;
                        } else {
                            // ...otherwise, wrap the value inside a *return value*
                            obj = Object::ReturnValue(Box::new(expr_eval));
                        }

                        break;
                    }

                    // evaluate all other types of statements
                    obj = self.eval_statement(statement)?;

                    // if the current object is a *return value*, stop evaluating this block
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

    fn eval_expression(
        &mut self,
        expr: Expression,
        within_statement: bool,
    ) -> Result<Object, EvalError> {
        let obj = match expr {
            Expression::IntegerLiteral(lit) => Object::IntegerValue(lit),
            Expression::BooleanLiteral(lit) => Object::BooleanValue(lit),
            Expression::StringLiteral(lit) => Object::StringValue(lit),
            Expression::Identifier(name) => self.env.borrow().get(&name)?,
            Expression::ArrayLiteral(expressions) => self.eval_array_expression(expressions)?,
            Expression::MapLiteral(map) => self.eval_map_expression(map)?,
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => self.eval_binary_expression(*left, operator, *right)?,
            Expression::UnaryExpression { operator, value } => {
                self.eval_unary_expression(operator, *value)?
            }
            Expression::GroupedExpression(expr) => self.eval_expression(*expr, within_statement)?,
            Expression::CallExpression { path, arguments } => {
                self.eval_call_expression(*path, arguments)?
            }
            Expression::IndexExpression { value, index } => {
                self.eval_index_expression(*value, *index)?
            }
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => self.eval_if_expression(*condition, *consequence, alternative)?,
            Expression::FunctionExpression { parameters, body } => {
                self.eval_function_expression(parameters, *body)?
            }
        };

        // unwrap return values
        if let Object::ReturnValue(ref inner_obj) = obj {
            if !within_statement {
                return Ok(*inner_obj.clone());
            }
        }

        Ok(obj)
    }

    fn eval_binary_expression(
        &mut self,
        left: Expression,
        operator: TokenKind,
        right: Expression,
    ) -> Result<Object, EvalError> {
        let left_obj = self.eval_expression(left, false)?;
        let right_obj = self.eval_expression(right, false)?;

        let obj = match (left_obj, right_obj) {
            (Object::IntegerValue(lhs), Object::IntegerValue(rhs)) => match operator {
                TokenKind::Plus => Object::IntegerValue(lhs + rhs),
                TokenKind::Minus => Object::IntegerValue(lhs - rhs),
                TokenKind::Asterisk => Object::IntegerValue(lhs * rhs),
                TokenKind::Equal => Object::BooleanValue(lhs == rhs),
                TokenKind::NotEqual => Object::BooleanValue(lhs != rhs),
                TokenKind::LessThan => Object::BooleanValue(lhs < rhs),
                TokenKind::GreaterThan => Object::BooleanValue(lhs > rhs),
                TokenKind::LessThanEqual => Object::BooleanValue(lhs <= rhs),
                TokenKind::GreaterThanEqual => Object::BooleanValue(lhs >= rhs),
                TokenKind::Percentage => {
                    if rhs == 0 {
                        return Err(EvalError::ModuloByZero);
                    } else {
                        Object::IntegerValue(lhs % rhs)
                    }
                }
                TokenKind::Slash => {
                    if rhs == 0 {
                        return Err(EvalError::DivisionByZero);
                    } else {
                        Object::IntegerValue(lhs / rhs)
                    }
                }
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            (Object::BooleanValue(lhs), Object::BooleanValue(rhs)) => match operator {
                TokenKind::Equal => Object::BooleanValue(lhs == rhs),
                TokenKind::NotEqual => Object::BooleanValue(lhs != rhs),
                TokenKind::AndAnd => Object::BooleanValue(lhs && rhs),
                TokenKind::OrOr => Object::BooleanValue(lhs || rhs),
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            (Object::StringValue(lhs), Object::StringValue(rhs)) => match operator {
                TokenKind::Plus => Object::StringValue(lhs + &rhs),
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            (lhs, rhs) => {
                return Err(EvalError::TypeMismatch(format!(
                    "Cannot perform operation '{operator}' between '{lhs}' and '{rhs}'",
                )));
            }
        };

        Ok(obj)
    }

    fn eval_unary_expression(
        &mut self,
        operator: TokenKind,
        value: Expression,
    ) -> Result<Object, EvalError> {
        let obj = match operator {
            TokenKind::Bang => match self.eval_expression(value, false)? {
                Object::IntegerValue(lit) => Object::IntegerValue(!lit),
                Object::BooleanValue(lit) => Object::BooleanValue(!lit),
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            TokenKind::Minus => match self.eval_expression(value, false)? {
                Object::IntegerValue(lit) => Object::IntegerValue(-lit),
                _ => return Err(EvalError::UnsupportedOperator(operator)),
            },

            _ => return Err(EvalError::UnsupportedOperator(operator)),
        };

        Ok(obj)
    }

    fn eval_array_expression(&mut self, expressions: Vec<Expression>) -> Result<Object, EvalError> {
        let mut objects: Vec<Object> = vec![];

        for expr in expressions {
            objects.push(self.eval_expression(expr, false)?);
        }

        Ok(Object::ArrayValue(objects))
    }

    fn eval_map_expression(
        &mut self,
        expr_map: HashMap<String, Expression>,
    ) -> Result<Object, EvalError> {
        let mut map: HashMap<String, Object> = HashMap::new();

        for (key, expr) in expr_map {
            map.insert(key, self.eval_expression(expr, false)?);
        }

        Ok(Object::MapValue(map))
    }

    fn eval_index_expression(
        &mut self,
        value: Expression,
        index: Expression,
    ) -> Result<Object, EvalError> {
        let value = self.eval_expression(value, false)?;
        let index = self.eval_expression(index, false)?;

        match value {
            Object::ArrayValue(objects) => {
                if let Object::IntegerValue(index) = index {
                    let id = usize::try_from(index)
                        .or_else(|err| return Err(ParserError::IntConversionError(err)))?;

                    let item = objects
                        .get(id)
                        .ok_or(EvalError::IndexOutOfBounds(objects.len(), id))?;

                    Ok(item.clone())
                } else {
                    return Err(EvalError::InvalidIndexType);
                }
            }
            Object::MapValue(map) => {
                if let Object::StringValue(key) = index {
                    let item = map.get(&key).ok_or(EvalError::ValueNotFound(key))?;

                    Ok(item.clone())
                } else {
                    return Err(EvalError::InvalidIndexType);
                }
            }
            _ => return Err(EvalError::InvalidIndexUsage),
        }
    }

    fn eval_if_expression(
        &mut self,
        condition: Expression,
        consequence: Statement,
        alternative: Option<Box<Statement>>,
    ) -> Result<Object, EvalError> {
        let obj = match self.eval_expression(condition, false)? {
            Object::BooleanValue(lit) => {
                if lit {
                    self.eval_statement(consequence)?
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
        body: Statement,
    ) -> Result<Object, EvalError> {
        let closure = Closure {
            parameters,
            body,
            env: self.create_enclosed_env(),
        };

        Ok(Object::FunctionValue(closure))
    }

    fn eval_call_expression(
        &mut self,
        path: Expression,
        arguments: Vec<Expression>,
    ) -> Result<Object, EvalError> {
        let function = match path {
            Expression::Identifier(path) => {
                // built-in functions are searched through before user-defined ones
                BuiltinFunction::lookup_function(&path).or_else(|_| self.env.borrow().get(&path))?
            }
            expr => self.eval_expression(expr, false)?,
        };

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

                // evaluate arguments in the current scope
                let arguments = self.eval_call_expression_arguments(arguments)?;

                // switch to the closure environment
                let outer_env = std::mem::replace(&mut self.env, env);

                // add bindings in the closure environment
                for (param, arg) in parameters.into_iter().zip(arguments.into_iter()) {
                    self.env.borrow_mut().set(param, arg);
                }

                // evaluate the closure body
                let body_obj = self.eval_statement(body)?;
                // go back to the old environment
                self.env = outer_env;

                body_obj
            }

            Object::BuiltinValue(builtin) => match builtin {
                BuiltinFunction::Len => {
                    if arguments.len() != 1 {
                        return Err(EvalError::FunctionCallWrongArity(1, arguments.len() as u8));
                    }

                    let arguments = self.eval_call_expression_arguments(arguments)?;
                    // unwrapping is fine, this element surely exist because of the previous check
                    let arg = arguments.get(0).unwrap();

                    let length: i32 = match arg {
                        Object::StringValue(text) => text
                            .len()
                            .try_into()
                            .or_else(|err| return Err(ParserError::IntConversionError(err)))?,

                        Object::ArrayValue(objects) => objects
                            .len()
                            .try_into()
                            .or_else(|err| return Err(ParserError::IntConversionError(err)))?,

                        _ => {
                            return Err(EvalError::UnsupportedArgumentType(format!(
                                "`{}` only retrieves the length of strings and arrays",
                                BuiltinFunction::Len
                            )));
                        }
                    };

                    Object::IntegerValue(length)
                }

                BuiltinFunction::Append => {
                    if arguments.len() < 2 {
                        return Err(EvalError::FunctionCallWrongArity(2, arguments.len() as u8));
                    }

                    let mut arguments = self.eval_call_expression_arguments(arguments)?;
                    let (first, rest) = arguments.split_first_mut().unwrap();

                    if let Object::ArrayValue(objects) = first {
                        objects.extend_from_slice(rest);
                        // return a new array, rather than modifying the existing one
                        Object::ArrayValue(objects.clone())
                    } else {
                        return Err(EvalError::UnsupportedArgumentType(format!(
                            "`{}` only works on arrays",
                            BuiltinFunction::Append
                        )));
                    }
                }

                BuiltinFunction::Rest => {
                    if arguments.len() != 1 {
                        return Err(EvalError::FunctionCallWrongArity(1, arguments.len() as u8));
                    }

                    let arguments = self.eval_call_expression_arguments(arguments)?;
                    // unwrapping is fine, this element surely exist because of the previous check
                    let arg = arguments.get(0).unwrap();

                    if let Object::ArrayValue(objects) = arg {
                        // return a new array, rather than modifying the existing one
                        Object::ArrayValue(objects[1..].to_vec())
                    } else {
                        return Err(EvalError::UnsupportedArgumentType(format!(
                            "`{}` only works on arrays",
                            BuiltinFunction::Append
                        )));
                    }
                }

                BuiltinFunction::Println => {
                    let arguments = self.eval_call_expression_arguments(arguments)?;
                    arguments.iter().for_each(|arg| println!("{arg}"));
                    Object::UnitValue
                }
                BuiltinFunction::Print => {
                    let arguments = self.eval_call_expression_arguments(arguments)?;
                    arguments.iter().for_each(|arg| print!("{arg}"));
                    Object::UnitValue
                }
            },

            other => {
                return Err(EvalError::FunctionNotFound(format!(
                    "`{other}` cannot be called as a function"
                )));
            }
        };

        Ok(obj)
    }

    fn eval_call_expression_arguments(
        &mut self,
        arguments: Vec<Expression>,
    ) -> Result<Vec<Object>, EvalError> {
        Ok(arguments
            .into_iter()
            .map(|arg| self.eval_expression(arg, false))
            .collect::<Result<Vec<Object>, EvalError>>()?)
    }

    /// Creates a new environment linked to the outer environment
    fn create_enclosed_env(&mut self) -> Rc<RefCell<Environment>> {
        let inner_env = Environment {
            outer: Some(self.env.clone()),
            ..Default::default()
        };
        Rc::new(RefCell::new(inner_env))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_integer_literal() {
        let input = "5";
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[0];
        assert_eq!(result, &Object::IntegerValue(5));
    }

    #[test]
    fn eval_boolean_literal() {
        let input = "true";
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[0];
        assert_eq!(result, &Object::BooleanValue(true));
    }

    #[test]
    fn eval_string_literal() {
        let input = r#""foo""#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[0];
        assert_eq!(result, &Object::StringValue("foo".to_owned()));
    }

    #[test]
    fn eval_string_concatenation() {
        let input = r#"
            let greet = fn() {
                let a = "hello";
                let b = "world";
                return a + " " + b;
            };

            greet();
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[1];
        assert_eq!(result, &Object::StringValue("hello world".to_owned()));
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
            ("true && true", true),
            ("true && false", false),
            ("false && true", false),
            ("false && false", false),
            ("true || true", true),
            ("true || false", true),
            ("false || true", true),
            ("false || false", false),
        ];

        for (input, expected) in tests {
            let mut evaluator = Evaluator::new(input);
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
            let mut evaluator = Evaluator::new(input);
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
            let mut evaluator = Evaluator::new(input);
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
            let mut evaluator = Evaluator::new(input);
            let result = &evaluator.eval_program().unwrap()[0];
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn eval_array_expression() {
        let input = r#"
            let add = fn(x, y) { return x + y; };
            [1 + 1, add(2, 2)];
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[1];
        assert_eq!(
            result,
            &Object::ArrayValue(vec![Object::IntegerValue(2), Object::IntegerValue(4)])
        );
    }

    #[test]
    fn eval_map_expression() {
        let input = r#"
            let foo = { "temp": 1 + 1, "foo": 4 };
            foo;
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[1];
        let mut expected = HashMap::new();
        expected.insert("temp".to_owned(), Object::IntegerValue(2));
        expected.insert("foo".to_owned(), Object::IntegerValue(4));
        assert_eq!(result, &Object::MapValue(expected));
    }

    #[test]
    fn eval_index_expression() {
        let input = r#"
            let a = [100, 200, 300, 400];
            1 + a[1 + 1];

            let b = { 
                "foo": 2,
                "bar": 4
            };
            b["foo"];
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = evaluator.eval_program().unwrap();
        assert_eq!(&result[1], &Object::IntegerValue(301));
        assert_eq!(&result[3], &Object::IntegerValue(2));
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
        let mut evaluator = Evaluator::new(input);
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
            ("1; fn(x) { x; }(5)", 5),
        ];

        for (input, expected) in tests {
            let mut evaluator = Evaluator::new(input);
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
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[2];
        assert_eq!(result, &Object::IntegerValue(2));
    }

    #[test]
    fn eval_assign_statement() {
        let input = r#"
            let a = 2;
            a = a + 2;
            a;
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[2];
        assert_eq!(result, &Object::IntegerValue(4));
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
        let mut evaluator = Evaluator::new(input);
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
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(&result[2], &Object::IntegerValue(4));
    }

    #[test]
    fn eval_nested_returns() {
        let input = r#"
            let add = fn(x, y) { return x + y; };

            let foo = fn() {
                return add(5 + 5, add(1, 1));
            };

            let faz = fn() {
                return 20;
            };

            let bar = if foo() == 12 {
                if foo() == 12 {
                    return faz();
                }

                return 100;
            } else {
                return -1;
            };

            bar;
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(&result[4], &Object::IntegerValue(20));
    }

    #[test]
    fn eval_function_as_parameter() {
        let input = r#"
            let add = fn(x, y) { return x + y; };
            let sub = fn(x, y) { return x - y; };
            let applyFunc = fn(a, b, cb) {
                cb(a, b)
            };
            applyFunc(2, 2, add);
            applyFunc(10, 2, sub);
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(&result[3], &Object::IntegerValue(4));
        assert_eq!(&result[4], &Object::IntegerValue(8));
    }

    #[test]
    fn builtin_len() {
        let input = r#"
            len("hello");
            len("");
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(&result[0], &Object::IntegerValue(5));
        assert_eq!(&result[1], &Object::IntegerValue(0));
    }

    #[test]
    fn builtin_append() {
        let input = r#"
            append([1, 2, 3], 100, 200);
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(
            &result[0],
            &Object::ArrayValue(vec![
                Object::IntegerValue(1),
                Object::IntegerValue(2),
                Object::IntegerValue(3),
                Object::IntegerValue(100),
                Object::IntegerValue(200),
            ])
        );
    }

    #[test]
    fn builtin_rest() {
        let input = r#"
            rest([1, 2, 3]);
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(
            &result[0],
            &Object::ArrayValue(vec![Object::IntegerValue(2), Object::IntegerValue(3)])
        );
    }

    #[test]
    fn custom_map() {
        let input = r#"
            let map = fn(arr, f) {
                let iter = fn(arr, accumulated) {
                    if len(arr) == 0 {
                        accumulated
                    } else {
                        iter(rest(arr), append(accumulated, f(arr[0])));
                    }
                };

                iter(arr, []);
            };

            let arr = [1, 2, 3, 4];
            let double = fn(x) { x * 2 };
            map(arr, double);
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(
            &result[3],
            &Object::ArrayValue(vec![
                Object::IntegerValue(2),
                Object::IntegerValue(4),
                Object::IntegerValue(6),
                Object::IntegerValue(8)
            ])
        );
    }

    #[test]
    fn custom_reduce() {
        let input = r#"
            let reduce = fn(arr, initial, f) {
                let iter = fn(arr, result) {
                    if len(arr) == 0 {
                        result
                    } else {
                        iter(rest(arr), f(result, arr[0]));
                    }
                };

                iter(arr, initial);
            };

            let sum = fn(arr) {
                return reduce(arr, 0, fn(initial, el) { initial + el });
            };

            sum([1, 2, 3, 4, 5]);
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap();
        assert_eq!(&result[2], &Object::IntegerValue(15));
    }
}
