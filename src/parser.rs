use std::rc::Rc;

use crate::{
    ast::{Expression, ParserError, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub cur: Rc<Token>,
    pub next: Rc<Token>,
}

/// Represents the binding power of a token.
/// For example, the precedences of these operators:
/// a   +   b   *   c   *   d   +   e
///. 1   2   3   4   3   4   1   2
#[derive(Debug)]
pub enum Precedence {
    Infix(u8, u8),
    Prefix(u8),
    Postfix(u8),
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);

        let mut parser = Self {
            lexer,
            cur: Rc::new(Token {
                kind: TokenKind::Eof,
                literal: "".to_owned(),
            }),
            next: Rc::new(Token {
                kind: TokenKind::Eof,
                literal: "".to_owned(),
            }),
        };

        // consume two tokens to set `cur` and `next` correctly
        parser.eat_token();
        parser.eat_token();

        parser
    }

    pub fn eat_token(&mut self) {
        /*
            This is like doing...
            ```
            self.cur = self.next;
            self.next = self.lexer.next_token().into();
            ```
            ...but respecting the borrow checker.
        */
        self.cur = std::mem::replace(&mut self.next, self.lexer.next_token().into());
    }

    pub fn expect_token(&mut self, token_kind: TokenKind) -> Result<Rc<Token>, ParserError> {
        if self.next.kind != token_kind {
            return Err(ParserError::UnexpectedToken(self.next.clone()));
        }

        self.eat_token();
        Ok(self.cur.clone())
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut statements: Vec<Statement> = vec![];

        while self.cur.kind != TokenKind::Eof {
            statements.push(self.parse_statement()?);
            self.eat_token();
        }

        Ok(Program(statements))
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.cur.kind {
            TokenKind::Let => self.parse_var_statement(),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::LeftBrace => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_var_statement(&mut self) -> Result<Statement, ParserError> {
        let kind = self.cur.kind.clone();
        let name = self.expect_token(TokenKind::Identifier)?;
        self.expect_token(TokenKind::Assign)?;
        let expr = self.parse_expression(0, false)?;
        self.expect_token(TokenKind::Semicolon)?;

        Ok(Statement::VarStatement {
            kind,
            name: name.literal.clone(),
            value: expr,
        })
    }

    pub fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.parse_expression(0, false)?;
        self.expect_token(TokenKind::Semicolon)?;
        Ok(Statement::ReturnStatement(expr))
    }

    pub fn parse_block_statement(&mut self) -> Result<Statement, ParserError> {
        // consume {
        self.eat_token();
        let mut statements: Vec<Statement> = vec![];

        while self.cur.kind != TokenKind::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.eat_token();
        }

        Ok(Statement::BlockStatement(statements))
    }

    pub fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.parse_expression(0, true)?;

        // make semicolon optional
        if self.next.kind == TokenKind::Semicolon {
            self.eat_token();
        }

        Ok(Statement::ExpressionStatement(expr))
    }

    fn infix_precedence(op: &TokenKind) -> Option<Precedence> {
        match op {
            TokenKind::Equal | TokenKind::NotEqual => Some(Precedence::Infix(1, 2)),

            TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanEqual
            | TokenKind::GreaterThanEqual => Some(Precedence::Infix(3, 4)),

            TokenKind::Plus | TokenKind::Minus => Some(Precedence::Infix(5, 6)),

            TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percentage => {
                Some(Precedence::Infix(6, 7))
            }

            _ => None,
        }
    }

    fn postfix_precedence(op: &TokenKind) -> Option<Precedence> {
        match op {
            TokenKind::LeftSquare | TokenKind::LeftParen => Some(Precedence::Postfix(8)),
            _ => None,
        }
    }

    fn prefix_precedence(op: &TokenKind) -> Option<Precedence> {
        match op {
            TokenKind::Bang | TokenKind::Minus => Some(Precedence::Prefix(9)),
            _ => None,
        }
    }

    /// Expression parsing done through Pratt's algorithm:
    /// * `min_prec` - set the min precedence.
    /// * `skip_eating` - skip the initial token eating. Useful for parsing *expression statements* and *grouped expressions*.
    pub fn parse_expression(
        &mut self,
        min_prec: u8,
        skip_eating: bool,
    ) -> Result<Expression, ParserError> {
        if !skip_eating {
            self.eat_token();
        }

        let mut expr = match self.cur.kind {
            TokenKind::Integer => Expression::IntegerLiteral(self.cur.literal.parse::<i32>()?),
            TokenKind::True => Expression::BooleanLiteral(true),
            TokenKind::False => Expression::BooleanLiteral(false),
            TokenKind::String => Expression::StringLiteral(self.cur.literal.clone()),

            TokenKind::Identifier => {
                if self.next.kind == TokenKind::LeftParen {
                    self.parse_call_expression()?
                } else {
                    Expression::Identifier(self.cur.literal.clone())
                }
            }

            TokenKind::LeftSquare => {
                Expression::ArrayLiteral(self.parse_expression_list(TokenKind::RightSquare)?)
            }

            TokenKind::LeftParen => self.parse_grouped_expression()?,

            // parse unary expressions based on prefix token precedences
            TokenKind::Bang | TokenKind::Minus => self.parse_unary_expression()?,

            TokenKind::If => self.parse_if_expression()?,

            TokenKind::Function => self.parse_function_expression()?,

            _ => {
                return Err(ParserError::UnexpectedToken(self.cur.clone()));
            }
        };

        // Pratt parsing uses both a loop and recursion to handle grouping based on precedences.
        loop {
            // parse postfix expressions
            if let Some(Precedence::Postfix(postfix_prec)) =
                Self::postfix_precedence(&self.next.kind)
            {
                if postfix_prec < min_prec {
                    break;
                }

                self.eat_token();

                expr = match self.cur.kind {
                    TokenKind::LeftSquare => {
                        if self.next.kind == TokenKind::RightSquare {
                            return Err(ParserError::SyntaxError(
                                "Define a valid index to access this structure (e.g. array[0])."
                                    .to_owned(),
                            ));
                        }

                        let index = Box::new(self.parse_expression(min_prec, false)?);
                        self.expect_token(TokenKind::RightSquare)?;

                        Expression::IndexExpression {
                            value: Box::new(expr),
                            index,
                        }
                    }
                    _ => {
                        return Err(ParserError::UnexpectedToken(self.cur.clone()));
                    }
                };

                continue;
            }

            // parse binary expressions based on infix operators precedences
            if let Some(Precedence::Infix(left_prec, right_prec)) =
                Self::infix_precedence(&self.next.kind)
            {
                if left_prec < min_prec {
                    break;
                }

                self.eat_token();
                let operator = self.cur.kind.clone();

                expr = match self.cur.kind {
                    TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Slash
                    | TokenKind::Asterisk
                    | TokenKind::Percentage
                    | TokenKind::Equal
                    | TokenKind::NotEqual
                    | TokenKind::LessThan
                    | TokenKind::GreaterThan
                    | TokenKind::LessThanEqual
                    | TokenKind::GreaterThanEqual => {
                        let right = self.parse_expression(right_prec, false)?;

                        Expression::BinaryExpression {
                            left: Box::new(expr),
                            operator,
                            right: Box::new(right),
                        }
                    }
                    _ => {
                        return Err(ParserError::UnexpectedToken(self.cur.clone()));
                    }
                };

                continue;
            }

            break;
        }

        Ok(expr)
    }

    pub fn parse_call_expression(&mut self) -> Result<Expression, ParserError> {
        let path = Box::new(self.parse_expression(0, false)?);
        self.expect_token(TokenKind::LeftParen)?;
        let arguments = self.parse_expression_list(TokenKind::RightParen)?;
        Ok(Expression::CallExpression { path, arguments })
    }

    pub fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.eat_token();
        let expr = match self.cur.kind {
            TokenKind::RightParen => {
                return Err(ParserError::SyntaxError(
                    "Empty grouped expression '()' isn't allowed".to_owned(),
                ))
            }
            _ => {
                let subexpr = self.parse_expression(0, true)?;
                self.expect_token(TokenKind::RightParen)?;
                subexpr
            }
        };

        Ok(Expression::GroupedExpression(Box::new(expr)))
    }

    /// Parse comma separated list of expressions. Supports trailing commas before the final token.
    fn parse_expression_list(&mut self, end: TokenKind) -> Result<Vec<Expression>, ParserError> {
        let mut expressions: Vec<Expression> = vec![];

        while self.next.kind != end {
            expressions.push(self.parse_expression(0, false)?);

            if self.next.kind == TokenKind::Comma {
                self.eat_token();
            } else if self.next.kind != end {
                return Err(ParserError::SyntaxError(
                    "Expected comma between arguments".to_owned(),
                ));
            }
        }

        self.expect_token(end)?;

        Ok(expressions)
    }

    pub fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = self.cur.kind.clone();

        let Some(Precedence::Prefix(prefix_prec)) = Self::prefix_precedence(&self.cur.kind) else {
            unreachable!();
        };

        let value = Box::new(self.parse_expression(prefix_prec, false)?);

        Ok(Expression::UnaryExpression { operator, value })
    }

    pub fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        let condition = self.parse_expression(0, false)?;
        self.expect_token(TokenKind::LeftBrace)?;
        let consequence = self.parse_block_statement()?;

        let alternative = if self.next.kind == TokenKind::Else {
            self.eat_token();
            self.expect_token(TokenKind::LeftBrace)?;
            Some(Box::new(self.parse_block_statement()?))
        } else {
            None
        };

        Ok(Expression::IfExpression {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
        })
    }

    pub fn parse_function_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_token(TokenKind::LeftParen)?;

        let mut parameters: Vec<String> = vec![];
        while self.next.kind != TokenKind::RightParen {
            if self.next.kind != TokenKind::Identifier && self.next.kind != TokenKind::Comma {
                break;
            }

            self.expect_token(TokenKind::Identifier)?;
            parameters.push(self.cur.literal.clone());

            if self.next.kind == TokenKind::Comma {
                self.eat_token();
            }
        }

        self.expect_token(TokenKind::RightParen)?;
        self.expect_token(TokenKind::LeftBrace)?;
        let body = Box::new(self.parse_block_statement()?);

        Ok(Expression::FunctionExpression { parameters, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_var_statement() {
        let input = r#"
            let five = 5;
            let taken = false;
            let temp = taken;
            let seven = five + 2 * 1;
        "#;

        let num_vars = input.lines().count() - 2;
        let mut parser = Parser::new(input);

        (0..num_vars).for_each(|_| {
            parser.parse_var_statement().unwrap();
            parser.eat_token();
        });
    }

    #[test]
    fn parse_return_statement() {
        let input = r#"
            return token;
        "#;

        let mut parser = Parser::new(input);
        parser.parse_return_statement().unwrap();
    }

    #[test]
    fn parse_expression_statement() {
        let input = r#"
            a + 2 * 2
        "#;

        let mut parser = Parser::new(input);
        parser.parse_expression_statement().unwrap();
    }

    #[test]
    fn parse_block_statement() {
        let input = r#"
            { let a = 2; }

            { 2 + 2; }
        "#;

        let mut parser = Parser::new(input);
        parser.parse_block_statement().unwrap();
    }

    #[test]
    fn parse_program() {
        let input = r#"
            let a = 1;
            let b = a + 1;
            return a / b;
            a + b
        "#;

        let mut parser = Parser::new(input);
        parser.parse_program().unwrap();
    }

    #[test]
    fn operator_precedence() {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 > 4 != 3 < 4", "((5 > 4) != (3 < 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(1, 2 * 3, sum(6, 7 * 8))",
                "add(1, (2 * 3), sum(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
            // (
            //     "a * [1, 2, 3, 4][b * c] * d",
            //     "((a * ([1, 2, 3, 4][(b * c)])) * d)",
            // ),
            // (
            //     "add(a * b[2], b[1], 2 * [1, 2][1])",
            //     "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
            // ),
        ];

        for test in tests {
            let (input, expected) = test;
            let mut parser = Parser::new(input);
            let res = parser.parse_program().unwrap().to_string();
            assert_eq!(expected, res);
        }
    }

    #[test]
    fn parse_if_expression() {
        let input = r#"
            let a = if 2 * 2 > 1 {
                let a = 3;
                a
            } else {
                b
            };

            if true { 2 };
        "#;

        let mut parser = Parser::new(input);
        parser.parse_program().unwrap();
    }

    #[test]
    fn parse_function_expression() {
        let input = r#"
            let a = fn(arg) {
                let bar = 2;

                return fn(foo) {
                    bar
                };
            };
        "#;

        let mut parser = Parser::new(input);
        parser.parse_program().unwrap();
    }

    #[test]
    fn parse_array_expression() {
        let input = r#"
            [1, [3 + 3, fn(x) { x; }]]
        "#;

        let mut parser = Parser::new(input);
        parser.parse_program().unwrap();
    }
}
