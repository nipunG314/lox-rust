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
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
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

pub trait Expr: fmt::Display {}

// Creates an generic Expression Type for a given set of fields
//
// The structs need to be generic since some of the
// fields of certain Expression types are other
// Expression types. In structs where multiple
// Expression types are included as fields, each
// must be provided a separate generic parameter when
// expr!() is called.
//
// If an Expression type does not contain any other
// Expression type as a field, the generic
// parameters can be skipped.
//
// Parameters:
//
// $trt: The Expr trait which each new type must
// implement
//
// $e: Identifier of the new struct
//
// $($T:ident),+: The sequence of generic
// parameters that the struct accepts
//
// $($field:ident: $ty:ident),*: The sequence of
// fields and their types that will populate the struct

macro_rules! expr {
    ($trt:ident: $e:ident<$($T:ident),+> => $($field:ident: $ty:ident),*) => {
        pub struct $e<$($T: $trt,)+> {
            $(pub $field: $ty,)*
        }

        impl<$($T,)+> $trt for $e<$($T,)+> where $($T: $trt,)+ {}
    };
    ($trt:ident: $e:ident => $($field:ident: $ty:ident),*) => {
        pub struct $e {
            $(pub $field: $ty,)*
        }

        impl $trt for $e {}
    };
}

expr!(Expr: Binary<T, U> => left: T, op: Token, right: U);

impl<T, U> fmt::Display for Binary<T, U>
where
    T: Expr,
    U: Expr,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right)
    }
}

expr!(Expr: Grouping<T> => expression: T);

impl<T> fmt::Display for Grouping<T>
where
    T: Expr,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

expr!(Expr: Literal => value: Token);

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.lexeme)
    }
}

expr!(Expr: Unary<T> => op: Token, right: T);

impl<T> fmt::Display for Unary<T>
where
    T: Expr,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.op, self.right)
    }
}
