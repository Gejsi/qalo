use crate::{
    ast::{Expression, Statement},
    object::{EvalError, Object},
    parser::Parser,
    token::TokenKind,
};

#[derive(Debug)]
pub struct Evaluator<'a> {
    parser: Parser<'a>,
}

impl<'a> Evaluator<'a> {
    pub fn new(input: &'a str) -> Self {
        let parser = Parser::new(&input);

        Evaluator { parser }
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
            Statement::VarStatement { kind, name, value } => todo!(),
            Statement::ReturnStatement(expr) => todo!(),
            Statement::ExpressionStatement(expr) => self.eval_expression(expr),
            Statement::BlockStatement(statements) => todo!(),
        }
    }

    fn eval_expression(&mut self, expr: Expression) -> Result<Object, EvalError> {
        let obj = match expr {
            Expression::Identifier(_) => todo!(),
            Expression::IntegerLiteral(lit) => Object::Integer(lit),
            Expression::BooleanLiteral(lit) => Object::Boolean(lit),
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let left_eval = self.eval_expression(*left)?;
                let right_eval = self.eval_expression(*right)?;

                match (left_eval, right_eval) {
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
                            "Cannot perform operation between {left_value} and {right_value}",
                        )))
                    }
                }
            }
            Expression::UnaryExpression { operator, value } => todo!(),
            Expression::GroupedExpression(expr) => todo!(),
            Expression::CallExpression { path, arguments } => todo!(),
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => todo!(),
            Expression::FunctionExpression { parameters, body } => todo!(),
            Expression::Empty => return Err(EvalError::Unknown),
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
    fn eval_binary_expressions() {
        let input = r#"
            2 + 3;
            4 - 1;
            5 * 6;
            10 / 2;
            7 == 7;
            8 != 9;
            true == true;
            false != true;
        "#;
        let mut evaluator = Evaluator::new(&input);
        evaluator.eval_program().unwrap();
    }
}
