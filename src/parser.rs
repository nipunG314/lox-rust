use crate::{
    core::{
        Binary, Grouping, Literal, ParseError, ParseResult, Token, TokenType, TokenType::*, Unary,
    },
    error,
};
use std::{iter::Peekable, slice::Iter};

pub struct Parser<'a> {
    reader: Peekable<Iter<'a, Token>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        let reader = tokens.iter().peekable();

        Parser { reader, current: 0 }
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> ParseResult {
        self.expression()
    }

    fn expression(&mut self) -> ParseResult {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult {
        let mut expr = self.comparison()?;

        while let Some(token) = self.check_next_token(|token| match token.token_type {
            BangEqual | EqualEqual => true,
            _ => false,
        }) {
            let token = token.clone();
            let right_expr = self.comparison()?;
            expr = Box::new(Binary::new(expr, token, right_expr));
        }

        self.check_unexpected_expr()?;

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult {
        let mut expr = self.addition()?;

        while let Some(token) = self.check_next_token(|token| match token.token_type {
            Greater | GreaterEqual | Less | LessEqual => true,
            _ => false,
        }) {
            let token = token.clone();
            let right_expr = self.addition()?;
            expr = Box::new(Binary::new(expr, token, right_expr));
        }

        self.check_unexpected_expr()?;

        Ok(expr)
    }

    fn addition(&mut self) -> ParseResult {
        let mut expr = self.multiplication()?;

        while let Some(token) = self.check_next_token(|token| match token.token_type {
            Minus | Plus => true,
            _ => false,
        }) {
            let token = token.clone();
            let right_expr = self.multiplication()?;
            expr = Box::new(Binary::new(expr, token, right_expr));
        }

        self.check_unexpected_expr()?;

        Ok(expr)
    }

    fn multiplication(&mut self) -> ParseResult {
        let mut expr = self.unary()?;

        while let Some(token) = self.check_next_token(|token| match token.token_type {
            Slash | Star => true,
            _ => false,
        }) {
            let token = token.clone();
            let right_expr = self.unary()?;
            expr = Box::new(Binary::new(expr, token, right_expr));
        }

        self.check_unexpected_expr()?;

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult {
        if let Some(token) = self.check_next_token(|token| match token.token_type {
            Bang | Minus => true,
            _ => false,
        }) {
            let token = token.clone();
            let right_expr = self.unary()?;
            return Ok(Box::new(Unary::new(token, right_expr)));
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> ParseResult {
        if let Some(token) = self.check_next_token(|token| match token.token_type {
            True | False | Nil | Number(_) | Str(_) => true,
            _ => false,
        }) {
            return Ok(Box::new(Literal::new(token.token_type.clone())));
        }

        if let Some(_) = self.check_next_token(|token| token.token_type == LeftParen) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression.")?;
            return Ok(Box::new(Grouping::new(expr)));
        }

        if let Some(token) = self.reader.peek() {
            return Err(Parser::error(token, "Expected expression."));
        }
        Err(ParseError {})
    }

    // Checks the next token and if it satisfies a closure, consumes it
    fn check_next_token<T>(&mut self, f: T) -> Option<&Token>
    where
        T: Fn(&Token) -> bool,
    {
        if let Some(c) = self.reader.peek() {
            if f(*c) {
                return self.advance();
            } else {
                return None;
            }
        }
        None
    }

    fn check_unexpected_expr(&mut self) -> Result<(), ParseError> {
        if let Some(token) = self.check_next_token(|token| match token.token_type {
            True | False | Nil | Number(_) | Str(_) | LeftParen | Identifier => true,
            _ => false,
        }) {
            return Err(Parser::error(token, "Unexpected Expression"));
        }
        Ok(())
    }

    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.reader.next()
    }

    fn error(token: &Token, message: &str) -> ParseError {
        error::token_error(token, message);

        ParseError {}
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if let Some(next_token) = self.reader.peek() {
            let token = next_token.clone();
            if token.token_type == token_type {
                self.advance();
                return Ok(token);
            }

            Err(Parser::error(token, message))
        } else {
            Err(ParseError {})
        }
    }
}
