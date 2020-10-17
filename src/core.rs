use std::any;
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

type LoxObject = Box<dyn any::Any>;

pub trait Expr: fmt::Display {
    fn interpret(&self) -> RuntimeResult;
}

pub struct SyntaxError {}
pub struct ParseError {}
pub struct RuntimeError(pub Token);

impl RuntimeError {
    pub fn empty(token_type: TokenType) -> Self {
        let mut empty_token = Token::empty();
        empty_token.token_type = token_type;

        RuntimeError(empty_token)
    }
}

pub type SyntaxResult = Result<(), SyntaxError>;
pub type ParseResult = Result<Box<dyn Expr>, ParseError>;
pub type RuntimeResult = Result<LoxObject, RuntimeError>;

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
    };
}

expr!(Expr: Binary<T, U> => left: Box<T>, op: Token, right: Box<U>);

impl<T, U> Expr for Binary<T, U>
where
    T: Expr + ?Sized,
    U: Expr + ?Sized,
{
    fn interpret(&self) -> RuntimeResult {
        let left_object = self.left.interpret()?;
        let right_object = self.right.interpret()?;

        use TokenType::*;
        match self.op.token_type {
            Plus | Minus | Slash | Star | Greater | GreaterEqual | Less | LessEqual => {
                if let Plus = self.op.token_type {
                    let lvalue = left_object.downcast_ref::<String>();
                    let rvalue = right_object.downcast_ref::<String>();

                    if lvalue.is_some() && rvalue.is_some() {
                        let str1 = lvalue.unwrap();
                        let str2 = rvalue.unwrap();
                        let mut new_string = String::with_capacity(str1.len() + str2.len());
                        new_string.push_str(str1);
                        new_string.push_str(str2);

                        return Ok(Box::new(new_string));
                    }
                }

                let lvalue = left_object.downcast_ref::<f64>();
                let rvalue = right_object.downcast_ref::<f64>();

                if lvalue.is_none() || rvalue.is_none() {
                    return Err(RuntimeError(self.op.clone()));
                }

                let lvalue = lvalue.unwrap();
                let rvalue = rvalue.unwrap();

                match self.op.token_type {
                    Plus => Ok(Box::new(*lvalue + *rvalue)),
                    Minus => Ok(Box::new(*lvalue - *rvalue)),
                    Slash => Ok(Box::new(*lvalue / *rvalue)),
                    Star => Ok(Box::new(*lvalue * *rvalue)),
                    Greater => Ok(Box::new(*lvalue > *rvalue)),
                    GreaterEqual => Ok(Box::new(*lvalue >= *rvalue)),
                    Less => Ok(Box::new(*lvalue < *rvalue)),
                    LessEqual => Ok(Box::new(*lvalue <= *rvalue)),
                    _ => Err(RuntimeError(self.op.clone())),
                }
            }
            EqualEqual | BangEqual => {
                let mut ans = false;

                let lvalue = left_object.downcast_ref::<Option<bool>>();
                let rvalue = right_object.downcast_ref::<Option<bool>>();

                if lvalue.is_some() && rvalue.is_some() {
                    let lvalue = lvalue.unwrap();
                    let rvalue = rvalue.unwrap();

                    ans = match *lvalue {
                        None => match *rvalue {
                            None => true,
                            _ => ans,
                        },
                        Some(true) => match *rvalue {
                            Some(true) => true,
                            _ => ans,
                        },
                        Some(false) => match *rvalue {
                            Some(false) => true,
                            _ => ans,
                        },
                    };
                }

                let lvalue = left_object.downcast_ref::<String>();
                let rvalue = right_object.downcast_ref::<String>();

                if lvalue.is_some() && rvalue.is_some() {
                    let str1 = lvalue.unwrap();
                    let str2 = rvalue.unwrap();

                    ans = str1 == str2;
                }

                let lvalue = left_object.downcast_ref::<f64>();
                let rvalue = right_object.downcast_ref::<f64>();

                if lvalue.is_some() && rvalue.is_some() {
                    let num1 = lvalue.unwrap();
                    let num2 = rvalue.unwrap();

                    ans = num1 == num2;
                }

                if self.op.token_type == EqualEqual {
                    Ok(Box::new(ans))
                } else {
                    Ok(Box::new(!ans))
                }
            }
            _ => Err(RuntimeError(self.op.clone())),
        }
    }
}

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

impl<T> Expr for Grouping<T>
where
    T: Expr + ?Sized,
{
    fn interpret(&self) -> RuntimeResult {
        return Ok(Box::new(self.expression.interpret()?));
    }
}

impl<T> fmt::Display for Grouping<T>
where
    T: Expr + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

expr!(Expr: Literal => value: TokenType);

impl Expr for Literal {
    fn interpret(&self) -> RuntimeResult {
        if let TokenType::Number(num) = self.value {
            return Ok(Box::new(num));
        }
        if let TokenType::Str(string) = self.value.clone() {
            return Ok(Box::new(string));
        }

        match self.value {
            TokenType::True | TokenType::False | TokenType::Nil => {
                let mut option: Option<bool> = None;
                if self.value == TokenType::True {
                    option = Some(true);
                } else if self.value == TokenType::False {
                    option = Some(false);
                }

                Ok(Box::new(option))
            }
            _ => Err(RuntimeError::empty(self.value.clone())),
        }
    }
}

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

impl<T> Expr for Unary<T>
where
    T: Expr + ?Sized,
{
    fn interpret(&self) -> RuntimeResult {
        let right_object = self.right.interpret()?;
        match self.op.token_type {
            TokenType::Minus => match right_object.downcast::<f64>() {
                Ok(num) => Ok(Box::new(-1.0 * *num)),
                _ => Err(RuntimeError(self.op.clone())),
            },
            TokenType::Bang => match right_object.downcast::<bool>() {
                Ok(truth) => Ok(Box::new(truth)),
                _ => Err(RuntimeError(self.op.clone())),
            },
            _ => Err(RuntimeError(self.op.clone())),
        }
    }
}

impl<T> fmt::Display for Unary<T>
where
    T: Expr + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.op, self.right)
    }
}
