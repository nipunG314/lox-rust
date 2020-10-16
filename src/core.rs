use std::fmt;

#[derive(Clone, PartialEq, Debug)]
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
    EOF,
}

#[derive(Clone, Debug)]
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

    pub fn empty() -> Token {
        Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            line: 0,
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

pub struct ParseError {}

pub type ParseResult = Result<Box<dyn Expr>, ParseError>;

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
// parameters can be skipped. In this case, the ?Sized trait
// bound is not applied since the resulting expression
// will not be recursive.
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
    ($trt:ident: $e:ident<$($T:ident),+> => $($field:ident: $ty:ty),*) => {
        pub struct $e<$($T: $trt + ?Sized,)+> {
            $(pub $field: $ty,)*
        }

        impl<$($T,)+> $e<$($T,)+> where $($T: $trt + ?Sized,)+ {
            pub fn new($($field: $ty,)*) -> Self {
                Self {
                    $($field,)*
                }
            }
        }

        impl<$($T,)+> $trt for $e<$($T,)+> where $($T: $trt + ?Sized,)+ {}
    };
    ($trt:ident: $e:ident => $($field:ident: $ty:ident),*) => {
        pub struct $e {
            $(pub $field: $ty,)*
        }

        impl $e {
            pub fn new($($field: $ty,)*) -> Self {
                Self {
                    $($field,)*
                }
            }
        }

        impl $trt for $e {}
    };
}

expr!(Expr: Binary<T, U> => left: Box<T>, op: Token, right: Box<U>);

impl<T, U> fmt::Display for Binary<T, U>
where
    T: Expr + ?Sized,
    U: Expr + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right)
    }
}

expr!(Expr: Grouping<T> => expression: Box<T>);

impl<T> fmt::Display for Grouping<T>
where
    T: Expr + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

expr!(Expr: Literal => value: TokenType);

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let TokenType::Str(s) = &self.value {
            return write!(f, "{}", s);
        } else if let TokenType::Number(num) = &self.value {
            return write!(f, "{}", num);
        }
        write!(f, "")
    }
}

expr!(Expr: Unary<T> => op: Token, right: Box<T>);

impl<T> fmt::Display for Unary<T>
where
    T: Expr + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.op, self.right)
    }
}
