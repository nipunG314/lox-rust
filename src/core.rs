use std::iter::Peekable;
use std::str::Chars;

pub fn run_prompt() {
    println!("Implement REPL");
}

pub fn run_file(source: String) {
    println!("Scan and print Tokens");
}

#[derive(Debug)]
enum TokenType {
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
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

struct Scanner<'a> {
    source: &'a String,
    reader: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a String) -> Scanner<'a> {
        let reader = source.chars().peekable();

        Scanner {
            source,
            reader,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.reader.next()
    }

    // Checks the next symbol and if it satisfies a closure, consumes it
    fn check_next_symbol<T>(&mut self, f: T) -> Option<bool>
    where
        T: Fn(char) -> bool,
    {
        if let Some(c) = self.reader.peek() {
            if f(*c) {
                self.advance();
                return Some(true);
            } else {
                return Some(false);
            }
        }
        None
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>();
        self.tokens.push(Token::new(token_type, text, self.line));
    }
}
