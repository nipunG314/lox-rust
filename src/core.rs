use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    Str(String),
    Number(f64),
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

pub struct NextTokenInfo(pub char, pub TokenType, pub TokenType);

trait Expr: fmt::Display {}

struct Binary<T>
where
    T: Expr,
{
    left: T,
    op: Token,
    right: T,
}

impl<T> Binary<T>
where
    T: Expr,
{
    fn new(left: T, op: Token, right: T) -> Binary<T> {
        Binary { left, op, right }
    }
}

impl<T> fmt::Display for Binary<T>
where
    T: Expr,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

impl<T> Expr for Binary<T> where T: Expr {}

struct Grouping<T>
where
    T: Expr,
{
    expression: T,
}

impl<T> Grouping<T>
where
    T: Expr,
{
    fn new(expression: T) -> Grouping<T> {
        Grouping { expression }
    }
}

impl<T> fmt::Display for Grouping<T>
where
    T: Expr,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.expression)
    }
}

impl<T> Expr for Grouping<T> where T: Expr {}

struct Literal {
    value: Token,
}

impl Literal {
    fn new(value: Token) -> Literal {
        Literal { value }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.lexeme)
    }
}

impl Expr for Literal {}

struct Unary<T>
where
    T: Expr,
{
    op: Token,
    right: T,
}

impl<T> Unary<T>
where
    T: Expr,
{
    fn new(op: Token, right: T) -> Unary<T> {
        Unary { op, right }
    }
}

impl<T> fmt::Display for Unary<T>
where
    T: Expr,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.op, self.right)
    }
}

impl<T> Expr for Unary<T> where T: Expr {}
