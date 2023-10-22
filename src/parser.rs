use std::rc::Rc;

use crate::{
    ast::{CallExpressionArgument, Expression, ParserError, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub cur: Rc<Token>,
    pub next: Rc<Token>,
}

/// Represents the binding power of a token (e.g. operators).
/// For example:
/// a   +   b   *   c   *   d   +   e
///. 1   2   3   4   3   4   1   2
#[derive(Debug)]
pub enum Precedence {
    Infix(u8, u8),
    Prefix(u8),
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(&input);

        let mut parser = Self {
            lexer,
            cur: Rc::new(Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            }),
            next: Rc::new(Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
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
            ... but respecting the borrow checker.
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

        // make semicolons optional
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

            TokenKind::Asterisk | TokenKind::Slash => Some(Precedence::Infix(6, 7)),

            _ => None,
        }
    }

    fn prefix_precedence(op: &TokenKind) -> Option<Precedence> {
        match op {
            TokenKind::Bang | TokenKind::Minus => Some(Precedence::Prefix(8)),
            _ => None,
        }
    }

    /// Expression parsing done through Pratt's algorithm:
    /// * `min_prec` - set the min precedence.
    /// * `skip_eating` - skip the initial token eating. Useful for parsing *expression statements* and *grouped expressions*.
    fn parse_expression(
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

            TokenKind::Identifier => {
                if self.next.kind == TokenKind::LeftParen {
                    self.parse_call_expression()?
                } else {
                    Expression::Identifier(self.cur.literal.clone())
                }
            }

            TokenKind::LeftParen => self.parse_grouped_expression()?,

            // parse unary expressions based on prefix token precedences
            TokenKind::Bang | TokenKind::Minus => self.parse_unary_expression()?,

            _ => {
                return Err(ParserError::UnexpectedToken(self.cur.clone()));
            }
        };

        // Pratt parsing uses both a loop and recursion to handle grouping based on precedences.
        while let Some(Precedence::Infix(left_prec, right_prec)) =
            Self::infix_precedence(&self.next.kind)
        {
            // parse binary expressions based on infix operators precedences
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
        }

        Ok(expr)
    }

    // TODO: verify empty call expressions
    fn parse_call_expression(&mut self) -> Result<Expression, ParserError> {
        let path = self.cur.literal.clone();
        self.eat_token();

        let mut arguments: Vec<CallExpressionArgument> = vec![];

        while self.next.kind != TokenKind::RightParen {
            let name = self.expect_token(TokenKind::Identifier)?.literal.clone();
            self.expect_token(TokenKind::Colon)?;
            let value = self.parse_expression(0, false)?;
            arguments.push(CallExpressionArgument { name, value });

            if self.next.kind == TokenKind::Comma {
                self.eat_token();
            }
        }

        self.expect_token(TokenKind::RightParen)?;
        Ok(Expression::CallExpression { path, arguments })
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.eat_token();
        let expr = match self.cur.kind {
            TokenKind::RightParen => Expression::Empty,
            _ => {
                let subexpr = self.parse_expression(0, true)?;
                self.expect_token(TokenKind::RightParen)?;
                subexpr
            }
        };

        Ok(Expression::GroupedExpression(Box::new(expr)))
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = self.cur.kind.clone();

        let Some(Precedence::Prefix(prec)) = Self::prefix_precedence(&self.cur.kind) else {
            unreachable!();
        };

        let value = Box::new(self.parse_expression(prec, false)?);

        Ok(Expression::UnaryExpression { operator, value })
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
        let mut parser = Parser::new(&input);

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

        let mut parser = Parser::new(&input);
        parser.parse_return_statement().unwrap();
    }

    #[test]
    fn parse_expression_statement() {
        let input = r#"
            a + 2 * 2
        "#;

        let mut parser = Parser::new(&input);
        parser.parse_expression_statement().unwrap();
    }

    #[test]
    fn parse_block_statement() {
        let input = r#"
            { let a = 2; }

            { 2 + 2; }
        "#;

        let mut parser = Parser::new(&input);
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

        let mut parser = Parser::new(&input);
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
            (
                "a + add(first: b * c) + d",
                "((a + add(first: (b * c))) + d)",
            ),
            (
                "add(a: 1, b: 2 * 3, c: sum(first: 6, second: 7 * 8))",
                "add(a: 1, b: (2 * 3), c: sum(first: 6, second: (7 * 8)))",
            ),
            (
                "add(first: a + b + c * d / f + g)",
                "add(first: (((a + b) + ((c * d) / f)) + g))",
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
            let mut parser = Parser::new(&input);
            let res = parser.parse_program().unwrap().to_string();
            assert_eq!(expected, res);
        }
    }
}
