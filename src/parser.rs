use crate::{
    core::{Expr, Token},
    error,
};
use std::{iter::Peekable, slice::Iter};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    reader: Peekable<Iter<'a, Token>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        let reader = tokens.iter().peekable();

        Parser {
            tokens,
            reader,
            current: 0,
        }
    }
}
