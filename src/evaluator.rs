use crate::{
    ast::{Expression, Statement},
    object::{EvalError, Object},
    parser::Parser,
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
            Expression::BooleanLiteral(_) => todo!(),
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => todo!(),
            Expression::UnaryExpression { operator, value } => todo!(),
            Expression::GroupedExpression(_) => todo!(),
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
    fn evaluate_integer_literal() {
        let input = "5";
        let mut evaluator = Evaluator::new(&input);
        evaluator.eval_program().unwrap();
    }
}